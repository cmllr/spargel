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
    let all_posts = helpers::get_posts();
    let total_items_count = all_posts.len() as u32;
    let current_site: u32 = 1;

    let pagination = structs::pagination::Pagination::get(total_items_count, current_site, all_posts);
    print!("{}, {}, {}, {}", pagination.has_next, pagination.has_prev, pagination.cur_page, pagination.all_pages);

    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            posts: all_posts
        },
    )
}

#[get("/<id>/<_slug>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String, _slug: String) -> Template {
    let posts = helpers::get_posts();
    let post = posts.into_iter().find(|d| d.id == id).unwrap();
    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            posts: vec![post]
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
