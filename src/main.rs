use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::{TraceContextExt, TraceError, Tracer},
    Key, KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use rand::Rng;

// create a constant key
const RANDOM: Key = Key::from_static_str("random.value");

fn init_tracer() -> Result<trace::Tracer, TraceError> {
    // Initialise OTLP Pipeline
    opentelemetry_otlp::new_pipeline()
        .tracing() // create OTLP tracing pipeline
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic() // create GRPC layer 
                .with_endpoint("http://host.docker.internal:4317"), // GRPC OTLP Jaeger Endpoint
        )
        // Trace provider configuration 
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otlp-basic",
            )])),
        )
        .install_batch(runtime::Tokio) // configure a span exporter
}



fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}


#[tokio::main]
async fn main() -> Result<(), TraceError> {
    // set the Global Propagator
    global::set_text_map_propagator(TraceContextPropagator::new());

    // intialise the tracer
    let tracer = init_tracer().unwrap();

    // start a new active span
    tracer.in_span("generating number", |cx| {
        let span = cx.span();
        let num = gen_number();
        span.add_event(
            "opentel demo event Generating Number".to_string(),
            vec![Key::new("number").i64(num.into())],
        );

        // set an active span attribute
        span.set_attribute(RANDOM.i64(10));


        // start a new span
        tracer.in_span("generate another number", |cx| {
            let span = cx.span();
            let num = gen_number();
            span.add_event(
                "Generating Number".to_string(),
                vec![Key::new("number").i64(num.into())],
            )
        })
    });

    // gracefully shutdown the tracer
    global::shutdown_tracer_provider();
    Ok(())
}
