//! src/main.rs

use std::net::TcpListener;
use api::startup::run;
use api::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic si falla la lectura de configuración
    let configuration = get_configuration().expect("Failed to read configuration.");
    
    // Ya no usamos el puerto hard-coded 8000
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
//comenario