# Example of exporting logs to OpenTelemetry collector

The example demonstrates usage of OpenTelemetry exporter (based on gRPC) in an application written in Rust,   
showing the sent data in Jaeger as the OpenTelemetry collector.

To run the demo, do the following:  
1. Clone this repo.  
2. Install the prerequisites: [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [docker engine](https://docs.docker.com/engine/install/).  
3. In the main directory run:  
    `` docker build . ``
    `` docker compose up ``
4. Browse (in web browser) to http://localhost:16686/ to see the traces in Jaeger.

Note:
opentel directory contains a simple configuration of Jaeger and Opentel collector, to run it just run
`` docker compose up `` in the opentel directory

