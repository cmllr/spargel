#[macro_use]
extern crate rocket;
extern crate chrono;

use std::{fs::File, io::Read};

use rocket::State;
use rocket_dyn_templates::{context, Template};
use rocket::fs::FileServer;

mod structs;
mod helpers;
mod feed;

#[get("/?<page>")]
fn index(blog_context: &State<structs::blog::Blog>, page: Option<usize>) -> Template {
    let all_posts = helpers::get_posts();
    let total_items_count: usize = all_posts.len();
    let current_site: usize = page.unwrap_or(1);

    let pagination = structs::pagination::Pagination::get(total_items_count, current_site, all_posts);

    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            pagination: pagination,
            meta: blog_context.meta.clone()
        },
    )
}

#[get("/post/<id>/<_slug>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String, _slug: String) -> Template {
    let posts = helpers::get_posts();
    let post = posts.into_iter().find(|d| d.id == id).unwrap();
    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            p: post,
            meta: blog_context.meta.clone()
        },
    )
}



#[get("/feed")]
fn feed_url(blog_context: &State<structs::blog::Blog>) -> feed::FeedResponse {
   let posts = helpers::get_posts();
   let content = feed::get_feed(blog_context.inner().clone(), posts);
   content
}




#[launch]
fn rocket() -> _ {
    let mut file = File::open("blog.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let mut blog_context: structs::blog::Blog = serde_json::from_str(&buff).unwrap();
    blog_context.meta.insert(String::from("generator"), format!("{} {}", option_env!("CARGO_PKG_NAME").unwrap(), option_env!("CARGO_PKG_VERSION").unwrap()));


    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
        .mount("/", routes![index, post, feed_url])
        
        .manage(blog_context)
}
