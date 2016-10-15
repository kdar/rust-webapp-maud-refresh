extern crate sharedlib;
extern crate notify;
extern crate libc;
extern crate tiny_http;

use libc::c_char;
use std::ffi::{CStr, CString};
use sharedlib::{Lib, Func, Symbol};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use notify::debounce::Event;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tiny_http::{Server, Request, Response};

struct Templates {
  tpl: String,
  reload: Arc<AtomicBool>,
}

impl Templates {
  fn new() -> Templates {
    Templates {
      tpl: "".to_owned(),
      reload: Arc::new(AtomicBool::new(true)),
    }
  }

  fn run(&mut self) {
    let mut reload = self.reload.clone();

    thread::spawn(move || {
      let (tx, rx) = channel();
      let mut watcher: RecommendedWatcher = Watcher::debounced(tx, Duration::from_millis(500))
        .expect("failed to create debounced watcher");
      watcher.watch("templates/target/debug/templates.dll",
               RecursiveMode::NonRecursive)
        .unwrap();

      loop {
        match rx.recv() {
          Ok(event) => {
            match event {
              Event::Create(_) | Event::Write(_) => {
                reload.store(true, Ordering::Relaxed);
              }
              _ => (),
            };
          }
          Err(e) => println!("watch error {}", e),
        }
      }
    });
  }

  fn get(&mut self) -> String {
    if self.reload.compare_and_swap(true, false, Ordering::Relaxed) {
      unsafe {
        let path_to_lib = "templates/target/debug/templates.dll";
        let lib = Lib::new(path_to_lib).unwrap();
        let template_symbol: Func<extern "C" fn(name: *const c_char) -> *const c_char> =
          lib.find_func("template").unwrap();
        let template = template_symbol.get();
        let mut tpl = template(CString::new("index").unwrap().into_raw() as *const c_char);
        if *tpl != 0 {
          let tpl = CStr::from_ptr(tpl).to_str().unwrap();
          self.tpl = tpl.to_string();
        }
      };
    }

    self.tpl.clone()
  }
}

struct App {
  templates: Templates,
}

impl App {
  fn handle(&mut self, request: Request) {
    let response = Response::from_string(self.templates.get());

    let response = response.with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..],
                                                 &b"text/html; charset=utf8"[..])
        .unwrap());

    request.respond(response);
  }
}

fn main() {
  let mut t = Templates::new();
  t.run();
  let mut app = App { templates: t };
  let server = Server::http("0.0.0.0:1234").unwrap();

  for request in server.incoming_requests() {
    app.handle(request);
  }
}
