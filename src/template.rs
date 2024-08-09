use {
    crate::engine::SearchResults,
    rocket::Request,
    rocket_dyn_templates::{context, Template},
};

pub fn search(query: &str, result: SearchResults) -> Template {
    let context = context! {
        query,
        result,
    };

    Template::render("search", context)
}

pub fn not_found(request: &Request) -> Template {
    let context = context! { url: request.uri() };

    Template::render("not_found", context)
}
