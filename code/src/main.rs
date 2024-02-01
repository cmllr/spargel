/*
spargel - THE blog engine nobody asked for!
Copyright (C) 2024  Christoph Mueller

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#[macro_use]
extern crate rocket;
extern crate chrono;

use std::{fs::File, io::Read};

use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use structs::post::Post;

mod feed;
mod helpers;
mod structs;

#[get("/?<page>")]
fn index(blog_context: &State<structs::blog::Blog>, page: Option<usize>) -> Template {
    let posts = helpers::get_posts();
    let all_pages: Vec<Post> = posts.iter().filter(|p| p.is_page).cloned().collect();
    let all_posts: Vec<Post> = posts.iter().filter(|p| !p.is_page).cloned().collect();
    let total_items_count: usize = all_posts.len();
    let current_site: usize = page.unwrap_or(1);

    let pagination =
        structs::pagination::Pagination::get(total_items_count, current_site, all_posts);

    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            pagination: pagination,
            meta: blog_context.meta.clone(),
            all_pages: all_pages
        },
    )
}

#[get("/post/<id>/<_slug>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String, _slug: String) -> Template {
    let posts = helpers::get_posts();
    let all_pages: Vec<Post> = posts.iter().filter(|p| p.is_page).cloned().collect();
    let post = posts.into_iter().find(|d| d.id == id).unwrap();
    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            p: post.clone(),
            meta: blog_context.meta.clone(),
            html: post.html(),
            all_pages: all_pages
        },
    )
}

#[get("/feed")]
fn feed_url(blog_context: &State<structs::blog::Blog>) -> feed::FeedResponse {
    let posts = helpers::get_posts();
    let all_posts: Vec<Post> = posts.iter().filter(|p| !p.is_page).cloned().collect();
   
    let content = feed::get_feed(blog_context.inner().clone(), all_posts);
    content
}

#[launch]
fn rocket() -> _ {
    let mut file = File::open("blog.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let mut blog_context: structs::blog::Blog = serde_json::from_str(&buff).unwrap();
    blog_context.meta.insert(
        String::from("generator"),
        format!(
            "{} {}",
            option_env!("CARGO_PKG_NAME").unwrap(),
            option_env!("CARGO_PKG_VERSION").unwrap()
        ),
    );

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
        .mount("/", routes![index, post, feed_url])
        .manage(blog_context)
}
