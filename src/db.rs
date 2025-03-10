use postgres::{Client};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use crate::config::get_db_url;

pub fn connect_to_db() -> Result<Client, Box<dyn std::error::Error>> {
    let db_url = get_db_url();

    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let connector = MakeTlsConnector::new(connector);

    let client = Client::connect(&db_url, connector)?;
    
    println!("Successfully connected to database!");
    Ok(client)
}


