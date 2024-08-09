use {
    rocket::{
        fs::{relative, NamedFile},
        Request,
    },
    rocket_dyn_templates::{context, Template},
};

#[macro_use]
extern crate rocket;

mod engine;
mod template;

#[get("/search?<q>")]
async fn search(q: &str) -> Result<Template, String> {
    let result = engine::request(q)
        .await
        .map_err(|error| error.to_string())?;

    let context = context! {
        query: q,
        result
    };

    Ok(Template::render("search", context))
}

#[get("/main.css")]
async fn stylesheet() -> NamedFile {
    NamedFile::open(relative!("static/main.css"))
        .await
        .ok()
        .unwrap()
}

#[catch(404)]
fn not_found(request: &Request) -> Template {
    template::not_found(request)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", rocket::routes![search, stylesheet])
        .register("/", rocket::catchers![not_found])
}
