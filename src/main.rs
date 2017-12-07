#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate reqwest;
extern crate rocket_contrib;

use rocket_contrib::Template;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::content;
use std::fs::File;
use std::io::prelude::*;

const HEAD: &'static str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>Response</title>
    </head>
    <body>"#;

const TAIL: &'static str = r#"
    </body>
</html>"#;

#[derive(FromForm)]
struct MyForm {
    query: String,
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String, _> = HashMap::new();
    Template::render("search", &context)
}

#[post("/submit", data = "<form>")]
fn submit(form: Form<MyForm>) -> content::Html<String> {
    let page = format!("{}{}{}", HEAD, form.get().query, TAIL);
    if let Ok(mut f) = File::create("honeypot/html/test.html") {
        f.write_all(&page.as_bytes());
        if let Ok(mut resp) = reqwest::get("http://127.0.0.1:8080/test.html") {
            let mut content = String::new();
            resp.read_to_string(&mut content);
            content::Html(content)
        } else {
            content::Html(format!("{}{}{}", HEAD, "INTERNAL ERROR 1", TAIL))
        }
    } else {
        content::Html(format!("{}{}{}", HEAD, "INTERNAL ERROR 2", TAIL))
    }
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, submit])
        .launch();
}
