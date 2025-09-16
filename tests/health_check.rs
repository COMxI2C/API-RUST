//! tests/health_check.rs

use sqlx::{PgConnection, Connection};
use sqlx::query;
use std::net::TcpListener;
use api::startup::run;
use api::configuration::get_configuration;
use sqlx::PgPool;

pub struct TestApp{
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failet to read configuration");
    // the objetive of the following line its asigna a random name to the new
    // database, then i enter to file configuration, after to the item d
    //database and the item "database_name" and i assign the name given by Uuid.new_v4, which it is similar to 
    //"a3f2c1d2-8b9e-4f1a-9c3e-2a1d5f6e7b8c. This proccess it due every test use the same db, this is a problem, 
    //then every test create its own db. at the end it can be delete

    configuration.database.database_name = Uuid::new_v4().to_string();
    let conection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to postgres");

    let server = run(listener, conection_pool.clone())
    .expect("Failed to start server");

    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: conection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool{
    //1 create the database
    let mut conection = PgConnection::connect(&config.connection_string_without_db())
    .await.expect("shit, failed to connect to postgres")
    conection.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str());
    .await.expect("Oh no, failed to crate database nooooo");

    //migrate database
    let connection_pool = PgPool::connect(&config.connection_string_without_db())
    .await.expect("Failed to connect database postgres");

    sqlx::migrate!("./migrations")
    .await.expect("Failes to migrate database----->");

    conection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app().await;            
    
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/health_check", &app_address.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await;
    // Act
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app_address.db_pool)
    .await
    .expect("Failed to fetch saved subscription !!!!");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
