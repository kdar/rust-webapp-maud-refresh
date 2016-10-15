#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
#[macro_use]
extern crate lazy_static;
extern crate libc;

use std::collections::HashMap;
use libc::c_char;
use std::ffi::{CStr, CString};

mod index;

pub struct Script(&'static str);

impl maud::Render for Script {
  fn render(&self) -> maud::Markup {
    html! {
      script type="text/javascript" src=(self.0) {}
    }
  }
}

pub struct InlineScript(&'static str);

impl maud::Render for InlineScript {
  fn render(&self) -> maud::Markup {
    html! {
      script (maud::PreEscaped(self.0))
    }
  }
}

pub struct Style(&'static str);

impl maud::Render for Style {
  fn render(&self) -> maud::Markup {
    html! {
      link rel="stylesheet" type="text/css" href=(self.0) /
    }
  }
}

pub struct InlineStyle(&'static str);

impl maud::Render for InlineStyle {
  fn render(&self) -> maud::Markup {
    html! {
      style (maud::PreEscaped(self.0))
    }
  }
}

struct Closure<'a>(&'a Fn() -> String);

impl<'a> maud::Render for Closure<'a> {
  fn render(&self) -> maud::Markup {
    maud::PreEscaped(self.0())
  }
}

lazy_static! {
  static ref TEMPLATES: HashMap<&'static str, String> = {
    let mut m = HashMap::new();
    m.insert("index", index::tpl());
    m
  };
}

#[no_mangle]
pub fn template(name: *const c_char) -> *const c_char {
  let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
  if let Some(tpl) = TEMPLATES.get(name) {
    return CString::new(tpl.clone().into_bytes()).unwrap().into_raw();
  }
  0 as *const c_char
}