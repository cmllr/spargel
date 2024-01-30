#[macro_use]
extern crate rocket;
extern crate chrono;

use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::fs::File;
use std::io::Read;
mod structs;
mod helpers;

#[get("/")]
fn index(blog_context: &State<structs::blog::Blog>) -> Template {
    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            posts: helpers::group_posts(helpers::get_posts()),
            max_days: 30
        },
    )
}

#[get("/<id>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String) -> Template {
    let posts = helpers::get_posts();
    let post = posts.into_iter().find(|d| d.id == id).unwrap();
    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            posts: helpers::group_posts(vec![post])
        },
    )
}



#[launch]
fn rocket() -> _ {
    let mut file = File::open("blog.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let blog_context: structs::blog::Blog = serde_json::from_str(&buff).unwrap();

    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, post])
        .manage(blog_context)
}
