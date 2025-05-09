# servidor
Minimal HTTP Server in Rust - Used for learning purposes.

# Overview: What is an HTTP server, and why build one?
An HTTP server:

Listens on a TCP socket for incoming connections from clients (usually browsers, APIs, etc.).

Parses each client’s request according to the HTTP protocol (method, path, headers, body).

Processes that request—routing it to the code that implements your application logic.

Formats an HTTP response (status line, headers, body) and writes it back to the client.

Optionally handles many requests concurrently, keeps connections alive, supports TLS, and more.

Building one “by hand” with only the standard library teaches you the raw steps; then switching to an async framework shows how real-world servers scale.