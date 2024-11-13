# http_server

This project implements a simple multi-threaded web server in Rust, using a custom ThreadPool to handle incoming client requests concurrently. 
The server listens for HTTP requests on 127.0.0.1:7878 and processes each connection in a separate thread from the pool, allowing it to handle multiple requests efficiently. 

#Usage

```rust
cargo run
```
