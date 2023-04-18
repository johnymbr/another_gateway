# Another Gateway

This project is a personal project to develop a gateway using Rust Lang. 

Initially, I plan to use this Gateway to control flow among it and applications behind it. 

It will be possible to create new applications and new flows between the Another Gateway and the applications, and the Another Gateway will generate metrics about these flows and a rate limit for these applications or for the client that created these flows.

## Tracing OpenTelemetry:
To enable I will use this tutorial: https://broch.tech/posts/rust-tracing-opentelemetry/