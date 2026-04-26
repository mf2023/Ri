//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

use crate::core::{RiResult, RiError};
use crate::gateway::{RiGateway, RiGatewayConfig, RiGatewayRequest};
use hyper::{Body, Request as HyperRequest, Response as HyperResponse, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::collections::HashMap as FxHashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_rustls::rustls::ServerConfig;

pub struct RiGatewayServer {
    gateway: Arc<RiGateway>,
    config: Arc<RwLock<RiGatewayConfig>>,
    addr: SocketAddr,
    tls_config: Option<ServerConfig>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl RiGatewayServer {
    pub fn new(gateway: Arc<RiGateway>, config: Arc<RwLock<RiGatewayConfig>>, addr: SocketAddr) -> Self {
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

    pub async fn serve(&mut self) -> RiResult<()> {
        let addr = self.addr;
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        let gateway = self.gateway.clone();
        let config = self.config.clone();

        let service = make_service_fn(move |_conn| {
            let gateway = gateway.clone();
            let config = config.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: HyperRequest<Body>| {
                    Self::handle_request(req, gateway.clone(), config.clone())
                }))
            }
        });

        let server = Server::bind(&addr)
            .http1_pipeline_flush(true)
            .serve(service);

        let graceful = server.with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        });

        graceful.await.map_err(|e| RiError::Other(format!("Server error: {}", e)))
    }

    async fn handle_request(
        req: HyperRequest<Body>,
        gateway: Arc<RiGateway>,
        config: Arc<RwLock<RiGatewayConfig>>,
    ) -> Result<HyperResponse<Body>, Infallible> {
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

        let mut headers = FxHashMap::default();
        for (key, value) in req.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(key.as_str().to_string(), v.to_string());
            }
        }

        let query_params = {
            let uri = req.uri();
            let query = uri.query().unwrap_or("");
            let mut params = FxHashMap::default();
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

        let ri_request = RiGatewayRequest::new(
            method.clone(),
            path.clone(),
            headers,
            query_params,
            body,
            remote_addr.clone(),
        );

        let response = gateway.handle_request(ri_request).await;

        let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

        if config.read().await.enable_logging {
            let log_level = &config.read().await.log_level;
            match log_level.as_str() {
                "debug" => {
                    log::debug!(
                        target: "Ri.Gateway",
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
                        target: "Ri.Gateway",
                        "{} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        duration_ms
                    );
                }
                "warn" => {
                    log::warn!(
                        target: "Ri.Gateway",
                        "{} {} {} {}ms",
                        method,
                        path,
                        response.status_code,
                        duration_ms
                    );
                }
                "error" => {
                    log::error!(
                        target: "Ri.Gateway",
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

        let body = Body::from(response.body);
        Ok(hyper_response.body(body).unwrap_or_else(|_| HyperResponse::default()))
    }

    pub async fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for RiGatewayServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

pub fn load_tls_config(
    cert_path: &str,
    key_path: &str,
) -> RiResult<ServerConfig> {
    let cert = std::fs::read(cert_path)
        .map_err(|e| RiError::Config(format!("Failed to read TLS certificate: {}", e)))?;
    let key = std::fs::read(key_path)
        .map_err(|e| RiError::Config(format!("Failed to read TLS key: {}", e)))?;

    let cert_chain = tokio_rustls::rustls::Certificate(cert);
    let private_key = tokio_rustls::rustls::PrivateKey(key);

    let mut server_config = tokio_rustls::rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert_chain], private_key)
        .map_err(|e| RiError::Config(format!("Failed to build TLS config: {}", e)))?;

    server_config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];

    Ok(server_config)
}
