/// Setup tracing
pub fn setup_tracing() {
    let log_filter = std::env::var("RUST_LOG")
    .unwrap_or_else(|_| "tracing=info,crustchan=debug".to_owned());
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .with_env_filter(log_filter)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}
