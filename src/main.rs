use newsletter::configurations::get_configuration;
use newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    run(
        TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
            .expect("Failed to bind port 8000"),
    )?
    .await
}
