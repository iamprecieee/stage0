use stage0::create_app;
use stage0::errors::Result;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;

    let app = create_app();

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,stage0=debug,tower_http=info".into());

    Ok(tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .flatten_event(true)
                .with_current_span(true)
                .with_span_list(false)
                .with_file(false)
                .with_target(true)
                .with_line_number(false)
                .with_thread_ids(false)
                .with_thread_names(false),
        )
        .try_init()?)
}
