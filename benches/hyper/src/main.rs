extern crate hyper;

use hyper::{Decoder, Encoder, Next, HttpStream};
use hyper::server::{Server, Handler, Request, Response, HttpListener};

static PHRASE: &'static [u8] = b"hello world";

struct Hello;

impl Handler<HttpStream> for Hello {
    fn on_request(&mut self, _: Request<HttpStream>) -> Next {
        Next::write()
    }
    fn on_request_readable(&mut self, _: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }
    fn on_response(&mut self, response: &mut Response) -> Next {
        Next::write()
    }
    fn on_response_writable(&mut self, encoder: &mut Encoder<HttpStream>) -> Next {
        encoder.write(PHRASE).unwrap();
        Next::end()
    }
}

fn main() {
    let listener = HttpListener::bind(&"127.0.0.1:9976".parse().unwrap()).unwrap();

    let listener = listener.try_clone().unwrap();
    Server::new(listener)
        .handle(|_| Hello)
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
}