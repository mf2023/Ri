<div align="center">

# Glossary

**Version: 0.1.8**

**Last modified date: 2026-02-20**

This chapter defines technical terms and concepts used in the DMSC documentation to help you understand the content.

## A

</div>

### API Gateway

An API gateway is a server that acts as a unified entry point for API requests, providing routing, load balancing, rate limiting, circuit breaking, and other functions.

### Async/Await

An asynchronous programming model in Rust that allows writing non-blocking code, improving system concurrency capabilities.

### Authentication

Verification of user or system identity. DMSC supports authentication methods such as JWT and OAuth2.

<div align="center">

## B

</div>

### Builder Pattern

A design pattern for creating complex objects. DMSC uses `DMSCAppBuilder` to implement the builder pattern.

### Batch Processing

Combining multiple operations into a single batch for execution, reducing system calls and network overhead.

<div align="center">

## C

</div>

### Cache

Temporary storage of frequently accessed data to reduce access to backend systems. DMSC supports multiple cache backends.

### Cache Penetration

Querying for non-existent data, causing requests to directly reach backend systems.

### Cache Consistency

Ensuring that data in cache remains consistent with data in backend systems.

### Configuration Management

Managing application configuration, supporting multi-environment, hot reload, and other features.

### Containerization

Packaging applications and their dependencies into containers for environment isolation and quick deployment.

### Core Module

The foundational module of DMSC, providing runtime, error handling, and service context functionality.

<div align="center">

## D

</div>

### DMSC

Dunimd Middleware Service, a high-performance Rust middleware framework that unifies backend infrastructure.

### DMSCError

DMSC's unified error type, containing error codes, messages, and context information.

### DMSCResult

A result type alias defined as `Result<T, DMSCError>` to simplify error handling.

### DMSCAppBuilder

An application builder for configuring and building DMSC applications.

### DMSCServiceContext

Service context that provides access to all module functionality.

### DMSCModule

A module trait for creating custom synchronous modules.

### Distributed Tracing

Tracking request flow in distributed systems to help locate performance bottlenecks.

### Docker

An open-source containerization platform for building, running, and managing containers.

<div align="center">

## E

</div>

### Environment Variables

Variables set in the operating system for configuring applications.

### Error Handling

Processing errors in applications to ensure system reliability.

<div align="center">

## F

</div>

### Fault Tolerance

The ability of a system to continue normal operation when failures occur.

### File System

A system that manages files and directories. DMSC provides secure file system operations.

<div align="center">

## G

</div>

### Gateway

An entry point for systems that handles external requests and forwards them to internal services.

### Grafana

An open-source monitoring and visualization platform for displaying Prometheus metrics.

<div align="center">

## H

</div>

### Health Check

Periodic checks of service health status to ensure services are operating normally.

### Hooks

Custom logic executed at specific lifecycle stages.

### HTTPS

A secure HTTP protocol that encrypts data transmission using TLS/SSL.

<div align="center">

## I

</div>

### Initialization

The startup preparation phase for modules or applications.

### Inversion of Control

A design pattern where object creation and dependency management are delegated to a framework.

<div align="center">

## J

</div>

### JWT

JSON Web Token, a token format for authentication, containing user information and signatures.

<div align="center">

## K

</div>

### Kubernetes

An open-source container orchestration platform for automating deployment, scaling, and management of containerized applications.

<div align="center">

## L

</div>

### Load Balancing

Distributing requests across multiple servers to improve system availability and performance.

### Logging

Recording application runtime status and events.

### Log Level

Defines the importance of logs, including DEBUG, INFO, WARN, ERROR, etc.

<div align="center">

## M

</div>

### Middleware

Software layer between applications and operating systems that provides common functionality.

### Modular Architecture

System architecture that divides systems into independent modules, supporting on-demand composition and extension.

### Module

A functional unit of DMSC that provides domain-specific functionality.

### Mutex

Mutual exclusion lock for protecting shared resources, preventing data races from concurrent access.

<div align="center">

## N

</div>

### Non-blocking I/O

Allows applications to continue executing other tasks while waiting for I/O operations to complete.

<div align="center">

## O

</div>

### Observability

Understanding a system's internal state through logs, metrics, and tracing.

### OAuth2

An open authorization protocol that allows third-party applications to access user resources without sharing passwords.

### OpenTelemetry

An open-source observability framework that provides unified logging, metrics, and tracing solutions.

<div align="center">

## P

</div>

### Prometheus

An open-source monitoring and alerting system for collecting and storing time-series metrics.

### Priority

Module loading order, where higher values indicate higher priority.

<div align="center">

## Q

</div>

### Queue

Used for asynchronous task processing, achieving system decoupling and peak shaving.

<div align="center">

## R

</div>

### Rate Limiting

Restricting request rates to prevent system overload.

### Redis

An open-source in-memory database commonly used for caching, message queues, and session storage.

### Rust

A systems programming language that provides high performance, memory safety, and concurrency support.

<div align="center">

## S

</div>

### Service Mesh

Manages service-to-service communication, providing service discovery, load balancing, and other functions.

### Service Discovery

Automatic detection and registration of available service instances.

### SpanID

A basic unit in distributed tracing that represents the execution of an operation.

### Structured Logging

Logging using key-value pair format for easier log analysis and processing.

### Synchronous Programming

Code executes sequentially, with one operation completing before the next begins.

<div align="center">

## T

</div>

### TLS/SSL

Transport Layer Security/Secure Sockets Layer, used for encrypting network communications.

### Tokio

Rust's asynchronous runtime that provides async I/O and task scheduling.

### TraceID

A unique ID that identifies a complete request in distributed tracing.

### Transaction

A group of operations that either all succeed or all fail.

<div align="center">

## U

</div>

### Unwrapping

Extracting values from Result or Option types, which may cause program crashes.

<div align="center">

## V

</div>

### Virtual Machine

A software environment that simulates a physical computer, used for running operating systems and applications.

<div align="center">

## W

</div>

### W3C Trace Context

W3C distributed tracing context standard that defines the format and propagation of TraceID and SpanID.

### WebAssembly

A low-level programming language that can run in web browsers, providing near-native performance.

<div align="center">

## X

</div>

### XSS

Cross-site scripting attack where attackers inject malicious scripts into web pages to steal user data.

<div align="center">

## Y

</div>

### YAML

A human-readable data serialization format commonly used for configuration files.

<div align="center">

## Z

</div>

### Zero Copy

An I/O optimization technique that reduces memory copy overhead, improving performance.

<div align="center">

## Design Patterns

</div>

### Dependency Injection

A design pattern where object dependencies are injected into objects instead of objects creating their own dependencies.

### Single Responsibility Principle

A class or module should be responsible for only one specific function.

### Loose Coupling

Weak dependency relationships between modules, facilitating maintenance and extension.

### High Cohesion

Related functionality is concentrated within the same module, improving module maintainability.

<div align="center">

## Performance Optimization

</div>

### Connection Pool

Pre-creates and manages database connections to reduce connection establishment and destruction overhead.

### Memory Leak

Programs fail to release unused memory during operation, causing memory usage to continuously grow.

### Throughput

Number of requests processed by the system per unit of time.

### Latency

Time from when a request is sent to when a response is received.

<div align="center">

## Security

</div>

### Principle of Least Privilege

Users or systems are granted only the minimum permissions required to complete tasks.

### CSRF

Cross-site request forgery, where attackers use user identities to execute unauthorized operations.

### SQL Injection

Attackers inject malicious SQL code to compromise database security.

### Encryption

Converting data into ciphertext to prevent unauthorized access.

### Decryption

Converting ciphertext back to original data.

<div align="center">

## Deployment

</div>

### Rolling Update

Gradually replacing old service instances to avoid service interruption.

### Blue-Green Deployment

Running two versions of services simultaneously, achieving seamless updates by switching routes.

### Canary Deployment

Deploying new versions of services to a small subset of users for verification before full rollout.

### CI/CD

Continuous Integration/Continuous Deployment, automating build, test, and deployment processes to improve development efficiency.

<div align="center">

## Monitoring

</div>

### Metrics

Data used to measure system performance and status, such as CPU usage, memory usage, etc.

### Counter

Monotonically increasing metrics used to record event occurrence counts.

### Gauge

Metrics that can increase or decrease, used to record current values.

### Histogram

Records the distribution of values, such as request latency.

### Summary

Records value quantiles, such as P50, P95, P99 latency.

<div align="center">

## Caching

</div>

### Cache Hit

Requested data is found in cache.

### Cache Miss

Requested data is not found in cache, requiring retrieval from backend systems.

### Cache Eviction

Removing old data when cache is full to make space for new data.

### TTL

Time-to-live, the validity period of cached data, which automatically expires after expiration.

<div align="center">

## Asynchronous Programming

</div>

### Future

Represents the result of an asynchronous operation, which will eventually complete or fail.

### Task

An asynchronous code unit that is scheduled by the Tokio runtime.

### Spawn

Creates a new asynchronous task and schedules it for execution.

### Join

Waits for multiple asynchronous tasks to complete.

<div align="center">

## Database

</div>

### ACID

Atomicity, Consistency, Isolation, Durability - four properties of database transactions.

### NoSQL

Non-relational databases that do not use traditional relational models, suitable for large-scale data storage.

### SQL

Structured Query Language, used for managing relational databases.

### Index

Data structures that improve database query speed.

<div align="center">

## Network

</div>

### DNS

Domain Name System, converts domain names to IP addresses.

### TCP

Transmission Control Protocol, a reliable connection-oriented protocol.

### UDP

User Datagram Protocol, an unreliable connectionless protocol.

### HTTP

Hypertext Transfer Protocol, used for transferring web pages and data.

### REST

Representational State Transfer, an architectural style for designing Web APIs.

### gRPC

A high-performance, open-source remote procedure call framework based on HTTP/2.

<div align="center">

## Development Process

</div>

### Code Review

Reviewing code quality, security, and correctness to improve code quality.

### Unit Test

Testing the smallest units of code, such as functions or methods.

### Integration Test

Testing interactions between multiple components.

### End-to-End Test

Testing system functionality from user interface to backend services.

### Mock

Using simplified objects to replace real objects for testing.

### Stub

Mock objects that return preset values for testing.

<div align="center">

## Operating System

</div>

### Process

A running program instance.

### Thread

Execution unit within a process that shares process resources.

### CPU Bound

Program execution speed is primarily limited by CPU performance.

### I/O Bound

Program execution speed is primarily limited by I/O operation speed.

<div align="center">

## Other

</div>

### DevOps

Combination of development and operations, emphasizing automation and collaboration to improve software delivery speed and quality.

### SRE

Site Reliability Engineering, applying software engineering practices to operations work to improve system reliability.

### Chaos Engineering

Testing system fault tolerance by actively injecting failures.

### Microservices

Dividing applications into independent small services for easier development, deployment, and scaling.

### Monolith

All functionality contained within a single application.

### Serverless

A cloud computing model where developers do not need to manage servers, only write code.
