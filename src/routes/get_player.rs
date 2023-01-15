use std::collections::HashMap;

use adhesion::http_server::{self, HTTPResponse, HTTPStatus};

use crate::models::player::Player;

pub fn get_player_listener(
    _headers: &HashMap<&str, &str>,
    _body: &String,
    query_parameters: &HashMap<&str, &str>,
    pool: &r2d2_postgres::r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>,
) -> HTTPResponse {
    let body = match query_parameters.get("uid") {
        Some(uid) => {
            match pool.get().unwrap().query_one("SELECT * FROM player WHERE uid = $1", &[&uid]) {
                Ok(player) => String::from(format!("{:?}", Player::new(player))),
                Err(error) => {
                    println!("FAILED RETRIEVING PLAYER: {error}");
                    String::from("Failed retrieving user")}
            }
            
        }
        None => String::from("missing URL parameter \"uid\""),
    };

    // let player = pool.get().unwrap().query_one(format!("SELECT {} FROM player", query_parameters.get("uid")), &[]).unwrap();
    HTTPResponse {
        status: HTTPStatus {
            status: 200,
            reason: String::from("OK"),
        },
        headers: http_server::default_headers(&body),
        body,
    }
}
