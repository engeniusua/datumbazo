#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/")]
fn root() -> &'static str {
    "Hello World!"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![root])
    .launch();
}
