#![feature(proc_macro_hygiene, decl_macro, never_type)]

use crate::request::FromRequest;
use rocket::http::*;
use rocket::Outcome::Success;
use rocket::*;
use rocket_contrib::serve::StaticFiles;

use rocket_contrib::json::Json;
use serde_derive::Serialize;

#[derive(Debug)]
struct HeaderCount(usize);

impl<'a, 'r> FromRequest<'a, 'r> for HeaderCount {
   type Error = !;

   fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, !> {
      Success(HeaderCount(request.headers().len()))
   }
}

#[get("/")]
fn index() -> &'static str {
   "Hello, world!"
}

#[derive(Serialize)]
struct Task {
   prop: String,
}

#[get("/todo")]
fn todo() -> Json<Task> {
   Json(Task {
      prop: String::from("test"),
   })
}

#[get("/hello/<name>")]
fn hello(name: &RawStr, header_count: HeaderCount) -> String {
   let string1 = String::from("long string is long");

   {
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {}", result);
   }
   return format!(
      "Hello, {} with header count {}!",
      name.as_str(),
      header_count.0
   );
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   if x.len() > y.len() {
      x
   } else {
      y
   }
}

#[get("/hello/<name>/<age>/<cool>")]
fn other_hello(name: String, age: u8, cool: bool) -> String {
   if cool {
      format!("You're a cool {} year old, {}!", age, name)
   } else {
      format!("{}, we need to talk about your coolness.", name)
   }
}

fn main() {
   rocket::ignite()
      .mount("/", routes![index, todo, hello, other_hello])
      .mount("/files", StaticFiles::from("static"))
      .launch();
}
