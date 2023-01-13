use adhesion::http_server::{HTTPListener, HTTPMethod, HTTPResponse, HTTPServer, HTTPStatus, Route, self};
use std::collections::HashMap;

fn main() {
    let server = HTTPServer {
        address: "127.0.0.1".to_string(),
        port: 8080,
        listeners: std::sync::Arc::new(HashMap::from([
            (
                Route {
                    method: HTTPMethod::GET,
                    location: String::from("/"),
                },
                root_listener as HTTPListener,
            ),
            (
                Route {
                    method: HTTPMethod::POST,
                    location: String::from("/que"),
                },
                test_listener as HTTPListener,
            ),
            (
                Route {
                    method: HTTPMethod::GET,
                    location: String::from("/nested/route"),
                },
                nested_listener as HTTPListener,
            )
        ])),
        default_404_listener: std::sync::Arc::new(None),
    };
    server.listen();
}

fn root_listener(
    _headers: &HashMap<&str, &str>,
    _body: &String,
    query_parameters: &HashMap<&str, &str>,
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

fn test_listener(
    _headers: &HashMap<&str, &str>,
    _body: &String,
    _query_parameters: &HashMap<&str, &str>,
) -> HTTPResponse {
    let body = format!("{:?}", _headers);
    HTTPResponse {
        status: HTTPStatus {
            status: 200,
            reason: String::from("OK"),
        },
        headers: http_server::default_headers(&body),
        body,
    }
}

fn nested_listener(
    _headers: &HashMap<&str, &str>,
    _body: &String,
    _query_parameters: &HashMap<&str, &str>,
) -> HTTPResponse {
    let body = format!("nested_listener response.\n{:?}", _headers);
    HTTPResponse {
        status: HTTPStatus {
            status: 200,
            reason: String::from("OK"),
        },
        headers: http_server::default_headers(&body),
        body,
    }
}
