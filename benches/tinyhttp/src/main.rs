extern crate tiny_http;


fn main() {
    let server = tiny_http::Server::http("0.0.0.0:9975").unwrap();
    println!("Now listening on port 9975");

    for rq in server.incoming_requests() {
        let response = tiny_http::Response::from_string("hello world".to_string());
        let _ = rq.respond(response);
    }
}