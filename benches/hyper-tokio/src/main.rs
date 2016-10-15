extern crate hyper;
extern crate futures;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Server, Service, Request, Response /* , HttpListener */};

static PHRASE: &'static [u8] = b"hello world";

#[derive(Clone, Copy)]
struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        ::futures::finished(Response::new()
                // .header(ContentLength(PHRASE.len() as u64))
                .header(ContentType::plaintext())
                .body(PHRASE))
    }

    fn poll_ready(&self) -> ::futures::Async<()> {
        ::futures::Async::Ready(())
    }
}

fn main() {
    println!("Listening on http://127.0.0.1:9976");
    Server::http(&"127.0.0.1:9976".parse().unwrap())
        .unwrap()
        .handle(Hello)
        .unwrap();

}