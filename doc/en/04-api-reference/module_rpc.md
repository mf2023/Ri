<div align="center">

# Module RPC API Reference

**Version: 0.1.8**

**Last modified date: 2026-02-20**

The module_rpc module provides inter-module RPC (Remote Procedure Call) communication capabilities, supporting both synchronous and asynchronous method calls.

## Module Overview

</div>

The module_rpc module includes the following core components:

- **DMSCModuleRPC**: RPC coordinator managing endpoints and method calls
- **DMSCModuleClient**: Client for making RPC calls to other modules
- **DMSCModuleEndpoint**: Endpoint definition for a module's exposed methods
- **DMSCMethodCall**: RPC method call request
- **DMSCMethodResponse**: RPC method call response

<div align="center">

## Core Components

</div>

### DMSCModuleRPC

RPC coordinator responsible for managing all registered module endpoints and routing method calls.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create RPC coordinator | None | `Self` |
| `with_default_timeout(timeout)` | Set default timeout | `timeout: Duration` | `Self` |
| `register_endpoint(endpoint)` | Register module endpoint | `endpoint: DMSCModuleEndpoint` | `()` |
| `unregister_endpoint(module_name)` | Unregister module endpoint | `module_name: &str` | `()` |
| `get_endpoint(module_name)` | Get module endpoint | `module_name: &str` | `Option<Arc<DMSCModuleEndpoint>>` |
| `call_method(module_name, method_name, params, timeout_ms)` | Call method | `module_name: &str`, `method_name: &str`, `params: Vec<u8>`, `timeout_ms: Option<u64>` | `DMSCMethodResponse` |
| `list_registered_modules()` | List registered modules | None | `Vec<String>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use std::sync::Arc;

async fn example() -> DMSCResult<()> {
    // Create RPC coordinator
    let rpc = DMSCModuleRPC::new();

    // Create and register module endpoint
    let endpoint = DMSCModuleEndpoint::new("user_service");
    endpoint.register_method("get_user", |_params| {
        Ok(vec![b"user_data"])
    });

    rpc.register_endpoint(endpoint).await;

    // Call method
    let response = rpc.call_method(
        "user_service",
        "get_user",
        vec![],
        None
    ).await;

    if response.is_success() {
        println!("Response: {:?}", response.data);
    }

    Ok(())
}
```

### DMSCModuleClient

RPC client providing a convenient interface for method calls.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(rpc)` | Create RPC client | `rpc: Arc<DMSCModuleRPC>` | `Self` |
| `call(module_name, method_name, params)` | Call method (with default timeout) | `module_name: &str`, `method_name: &str`, `params: Vec<u8>` | `DMSCMethodResponse` |
| `call_with_timeout(module_name, method_name, params, timeout_ms)` | Call method (with specified timeout) | `module_name: &str`, `method_name: &str`, `params: Vec<u8>`, `timeout_ms: u64` | `DMSCMethodResponse` |

#### Usage Example

```rust
use dmsc::prelude::*;
use std::sync::Arc;

async fn client_example() -> DMSCResult<()> {
    let rpc = Arc::new(DMSCModuleRPC::new());
    
    // Create client
    let client = DMSCModuleClient::new(rpc);

    // Call method
    let response = client.call(
        "user_service",
        "get_user",
        vec![]
    ).await;

    // Call with custom timeout
    let response = client.call_with_timeout(
        "user_service",
        "get_user",
        vec![],
        3000  // 3 second timeout
    ).await;

    Ok(())
}
```

### DMSCModuleEndpoint

Module endpoint defining methods exposed by a module.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(module_name)` | Create module endpoint | `module_name: &str` | `Self` |
| `module_name()` | Get module name | None | `&str` |
| `register_method(name, handler)` | Register synchronous method | `name: &str`, `handler: Fn(Vec<u8>) -> DMSCResult<Vec<u8>>` | `&Self` |
| `register_method_async(name, handler)` | Register asynchronous method | `name: &str`, `handler: Fn(Vec<u8>) -> DMSCResult<Vec<u8>>` | `&Self` |
| `get_method(name)` | Get method | `name: &str` | `Option<DMSCMethodRegistration>` |
| `list_methods()` | List all methods | None | `Vec<String>` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Create endpoint
let endpoint = DMSCModuleEndpoint::new("order_service");

// Register methods
endpoint
    .register_method("create_order", |params| {
        // Handle create order logic
        Ok(vec![b"order_created"])
    })
    .register_method("cancel_order", |params| {
        // Handle cancel order logic
        Ok(vec![b"order_cancelled"])
    });

// List all methods
let methods = endpoint.list_methods().await;
println!("Available methods: {:?}", methods);
```

### DMSCMethodCall

RPC method call request structure.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `method_name` | `String` | Method name |
| `params` | `Vec<u8>` | Method parameters (serialized bytes) |
| `timeout_ms` | `u64` | Timeout in milliseconds, default 5000 |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(method_name, params)` | Create method call | `method_name: String`, `params: Vec<u8>` | `Self` |
| `with_timeout_ms(timeout_ms)` | Set timeout | `timeout_ms: u64` | `Self` |

### DMSCMethodResponse

RPC method call response structure.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `success` | `bool` | Whether the call was successful |
| `data` | `Vec<u8>` | Return data (serialized bytes) |
| `error` | `String` | Error message |
| `is_timeout` | `bool` | Whether the call timed out |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create empty response | None | `Self` |
| `success_data(data)` | Create success response | `data: Vec<u8>` | `Self` |
| `error_msg(msg)` | Create error response | `msg: String` | `Self` |
| `timeout()` | Create timeout response | None | `Self` |
| `is_success()` | Check if successful | None | `bool` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Create success response
let response = DMSCMethodResponse::success_data(vec![1, 2, 3]);
assert!(response.is_success());

// Create error response
let response = DMSCMethodResponse::error_msg("Invalid parameter".to_string());
assert!(!response.is_success());
assert_eq!(response.error, "Invalid parameter");

// Create timeout response
let response = DMSCMethodResponse::timeout();
assert!(response.is_timeout);
```

<div align="center">

## Design Principles

</div>

1. **Type Safety**: All RPC calls are type-safe with proper serialization
2. **Async Support**: Both synchronous and asynchronous RPC calls are supported
3. **Timeout Control**: Configurable timeouts for all RPC calls
4. **Error Handling**: Comprehensive error handling with specific error types
5. **Thread Safety**: All components are thread-safe using Arc and RwLock
6. **Module Isolation**: Each module has its own namespace for methods

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [queue](./queue.md): Message queue module providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
