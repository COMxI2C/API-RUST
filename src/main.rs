//! src/main.rs

use std::net::TcpListener;
use api::startup::run;
use api::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic si falla la lectura de configuración
    let configuration = get_configuration().expect("Failed to read configuration.");
    //create the instance Pgpool
    let conection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to postgres!!!");
    
    // Ya no usamos el puerto hard-coded 8000
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, conection_pool)?.await?;
    Ok(())
}
//comenario

//funcion para llamar a un endpoint
//esto es una prueba
