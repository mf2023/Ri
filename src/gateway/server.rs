//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#![cfg(feature = "gateway")]
#![allow(non_snake_case)]

use crate::core::{DMSCResult, DMSCError};
use crate::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCGatewayRequest};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request as HyperRequest, Response as HyperResponse, StatusCode};
use hyper::body::Incoming;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;

pub struct DMSCGatewayServer {
    gateway: Arc<DMSCGateway>,
    config: Arc<RwLock<DMSCGatewayConfig>>,
    addr: SocketAddr,
    tls_config: Option<ServerConfig>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl DMSCGatewayServer {
    pub fn new(gateway: Arc<DMSCGateway>, config: Arc<RwLock<DMSCGatewayConfig>>, addr: SocketAddr) -> Self {
        Self {
            gateway,
            config,
            addr,
            tls_config: None,
            shutdown_tx: None,
        }
    }

    pub fn with_tls(mut self, tls_config: ServerConfig) -> Self {
        self.tls_config = Some(tls_config);
        self
    }

    pub async fn serve(&mut self) -> DMSCResult<()> {
        let addr = self.addr;
        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        let gateway = self.gateway.clone();
        let config = self.config.clone();

        let listener = TcpListener::bind(addr).await
            .map_err(|e| DMSCError::Other(format!("Failed to bind to {}: {}", addr, e)))?;

        log::info!(target: "DMSC.Gateway", "Server listening on {}", addr);

        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((stream, remote_addr)) => {
                            let gateway = gateway.clone();
                            let config = config.clone();
                            
                            tokio::spawn(async move {
                                let service = service_fn(move |req: HyperRequest<Incoming>| {
                                    let gateway = gateway.clone();
                                    let config = config.clone();
                                    async move {
                                        Self::handle_request(req, gateway, config).await
                                    }
                                });

                                let io = hyper_util::rt::TokioIo::new(stream);
                                
                                if let Err(e) = http1::Builder::new()
                                    .serve_connection(io, service)
                                    .await
                                {
                                    log::error!(target: "DMSC.Gateway", "Connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            log::error!(target: "DMSC.Gateway", "Accept error: {}", e);
                        }
                    }
                }
                
                _ = &mut shutdown_rx => {
                    log::info!(target: "DMSC.Gateway", "Server shutting down");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_request(
        req: HyperRequest<Incoming>,
        gateway: Arc<DMSCGateway>,
        config: Arc<RwLock<DMSCGatewayConfig>>,
    ) -> Result<HyperResponse<String>, Infallible> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let start = std::time::Instant::now();

        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let remote_addr = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                req.extensions()
                    .get::<SocketAddr>()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            });

        let mut headers = HashMap::new();
        for (key, value) in req.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(key.as_str().to_string(), v.to_string());
            }
        }

        let query_params = {
            let uri = req.uri();
            let query = uri.query().unwrap_or("");
            let mut params = HashMap::new();
            for pair in query.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    params.insert(
                        key.to_string(),
                        value.to_string(),
                    );
                }
            }
            params
        };

        let body = match hyper::body::to_bytes(req.into_body()).await {
            Ok(bytes) => {
                if bytes.is_empty() {
                    None
                } else {
                    Some(bytes.to_vec())
                }
            }
            Err(_) => None,
        };

        let dmsc_request = DMSCGatewayRequest::new(
            method.clone(),
            path.clone(),
            headers,
            query_params,
            body,
            remote_addr.clone(),
        );

        let response = gateway.handle_request(dmsc_request).await;

        let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

        if config.read().await.enable_logging {
            let log_level = &config.read().await.log_level;
            match log_level.as_str() {
                "debug" => {
                    log::debug!(
                        target: "DMSC.Gateway",
                        "{} {} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        request_id,
                        duration_ms
                    );
                }
                "info" => {
                    log::info!(
                        target: "DMSC.Gateway",
                        "{} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        duration_ms
                    );
                }
                "warn" => {
                    log::warn!(
                        target: "DMSC.Gateway",
                        "{} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        duration_ms
                    );
                }
                "error" => {
                    log::error!(
                        target: "DMSC.Gateway",
                        "{} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        duration_ms
                    );
                }
                _ => {}
            }
        }

        let mut hyper_response = HyperResponse::builder()
            .status(StatusCode::from_u16(response.status_code).unwrap_or(StatusCode::OK));

        for (key, value) in response.headers {
            if let (Ok(k), Ok(v)) = (key.parse::<hyper::header::HeaderName>(), value.parse::<hyper::header::HeaderValue>()) {
                hyper_response = hyper_response.header(k, v);
            }
        }

        let body = String::from_utf8_lossy(&response.body).to_string();
        Ok(hyper_response.body(body).unwrap_or_else(|_| HyperResponse::default()))
    }

    pub async fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for DMSCGatewayServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

pub fn load_tls_config(
    cert_path: &str,
    key_path: &str,
) -> DMSCResult<ServerConfig> {
    let cert = std::fs::read(cert_path)
        .map_err(|e| DMSCError::Config(format!("Failed to read TLS certificate: {}", e)))?;
    let key = std::fs::read(key_path)
        .map_err(|e| DMSCError::Config(format!("Failed to read TLS key: {}", e)))?;

    let cert_chain = tokio_rustls::rustls::Certificate(cert);
    let private_key = tokio_rustls::rustls::PrivateKey(key);

    let mut server_config = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert_chain], private_key)
        .map_err(|e| DMSCError::Config(format!("Failed to build TLS config: {}", e)))?;

    server_config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];

    Ok(server_config)
}
