use adhesion::http_server::{HTTPListener, HTTPMethod, HTTPServer, Route};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::collections::HashMap;

mod models;
mod routes;

fn main() {
    dotenv::dotenv().ok();
    let env_host = std::env::var("DB_HOST").expect("DB_HOST must be set.");
    let env_port = std::env::var("DB_PORT").expect("DB_PORT must be set.");
    let env_usr = std::env::var("DB_USR").expect("DB_USR must be set.");
    let env_db = std::env::var("DB_DB").expect("DB_DB must be set.");
    let env_pw = std::env::var("DB_PW").expect("DB_PW must be set.");

    let manager = PostgresConnectionManager::<NoTls>::new(
        format!("postgres://{env_usr}:{env_pw}@{env_host}:{env_port}/{env_db}")
            .parse()
            .unwrap(),
        NoTls,
    );

    let pool = r2d2_postgres::r2d2::Pool::new(manager).unwrap();

    // let c_ref: &'static Client = &client;

    for row in pool.get().unwrap().query("SELECT * FROM player", &[]).unwrap() {
        let id: String = row.get("uid");
        println!("{:?}", id);
    }

    let server = HTTPServer {
        address: "127.0.0.1".to_string(),
        port: 8080,
        listeners: std::sync::Arc::new(HashMap::from([
            (
                Route {
                    method: HTTPMethod::GET,
                    location: String::from("/player"),
                },
                routes::get_player::get_player_listener as HTTPListener<r2d2_postgres::r2d2::Pool<PostgresConnectionManager<NoTls>>>,
            ),
            (
                Route {
                    method: HTTPMethod::GET,
                    location: String::from("/"),
                },
                routes::root::root_listener,
            ),
        ])),
        default_404_listener: std::sync::Arc::new(None),
        threads: 5,
        passthrough: pool,
    };
    server.listen();
}
