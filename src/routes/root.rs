use std::collections::HashMap;

use adhesion::http_server::{HTTPResponse, HTTPStatus, self};

pub fn root_listener(
    _headers: &HashMap<&str, &str>,
    _body: &String,
    query_parameters: &HashMap<&str, &str>,
    _pool: &r2d2_postgres::r2d2::Pool<r2d2_postgres::PostgresConnectionManager<postgres::NoTls>>,
) -> HTTPResponse {
    // std::thread::sleep(std::time::Duration::from_secs(5));
    let body = format!("root listener.\n{:?}", query_parameters);
    HTTPResponse {
        status: HTTPStatus {
            status: 200,
            reason: String::from("OK"),
        },
        headers: http_server::default_headers(&body),
        body,
    }
}