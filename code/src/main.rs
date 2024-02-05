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
extern crate rand;
extern crate image;

use rand::distributions::{Alphanumeric, DistString};
use rocket::{data::ToByteUnit, response::Redirect};
use std::{fs::{self, File}, io::Read, path::Path};
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use structs::post::Post;
use rocket::form::Form;
use image::GenericImageView;

mod feed;
mod helpers;
mod structs;


#[get("/robots.txt")] 
fn robots_txt() -> String {
    let mut robots_content = String::from("User-Agent: *\n");
    let posts = helpers::get_posts();
    let hidden_posts: Vec<Post> = posts.iter().filter(|p| p.hide_from_robots).cloned().collect();

    for post in hidden_posts {
        robots_content.push_str(format!("Disallow: {}\n", post.url()).as_str());
    }
    
    return robots_content;
}

#[get("/?<tag>&<page>&<token>")]
fn index(blog_context: &State<structs::blog::Blog>, tag: Option<String>, page: Option<usize>, token: Option<String>) -> Template {
    let mut posts = helpers::get_posts();
   
    let all_pages: Vec<Post> = posts.iter().filter(|p| p.is_page).cloned().collect();
    if tag.is_some() {
        let tag_value = tag.unwrap();
        posts = posts.iter().filter(|p| p.tags.contains(&tag_value)).cloned().collect();
    }
    let all_posts: Vec<Post> = posts.iter().filter(|p| !p.is_page).cloned().collect();
    let total_items_count: usize = all_posts.len();
    let current_site: usize = page.unwrap_or(1);

    let pagination =
        structs::pagination::Pagination::get(total_items_count, current_site, all_posts);
    let mut is_edit_mode = false;
    if token.is_some() {
        if token.unwrap() == blog_context.token {
            is_edit_mode = true;
        }
    }
    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            pagination: pagination,
            meta: blog_context.meta.clone(),
            all_pages: all_pages,
            url: blog_context.url.to_owned(),
            is_edit_mode: is_edit_mode,
            token: blog_context.token.to_owned()
        },
    )
}


#[derive(FromForm)]
struct EditInput {
    path: String,
    raw_content: String,
    return_to: String,
    submit: String,
}

use rocket::Data;
#[post("/media?<token>&<size>&<name>", data = "<paste>")]
async fn media_upload(blog_context: &State<structs::blog::Blog>, token: String, size: u64, name: String, paste: Data<'_>) -> Redirect {
    // Check token
    if token != blog_context.token {
        return Redirect::to("/");
    }
    // TODO: That protection ist just RIDICULOUSLY BAD

    if helpers::is_ext_allowed(name.clone())
    {
        let path = Path::new("./uploads").join(name);
        let _ = paste.open(size.kibibytes()).into_file(path.clone()).await;
        let img = image::open(path.clone()).unwrap();
        let dim = img.dimensions();
        let (x, y) = dim;
        // TODO: MAKE configuration
        // Resize images larget than a desired width to prevent ridicoulous loading times
        if x > 1024 {
            let new_x = 1024;
            let new_y = y/(x/new_x);
            print!("Resizing to x={} and y={}", new_x, new_y);

            let new_image = img.resize(new_x, new_y, image::imageops::FilterType::Lanczos3);
            let _ = new_image.save(path.clone());
        }
    } else {
        return Redirect::to("/");
    }

    // TODO: Scale down images wider than 1024
    return Redirect::to("/media");
}

#[get("/media?<token>")]
fn media(blog_context: &State<structs::blog::Blog>, token: Option<String>) -> Template {
    let mut media_contents: Vec<String> = Vec::new();
    let paths = fs::read_dir("./uploads").unwrap();
    // TODO: make global-ish
    let allowed_ext = vec![".png", ".jpeg", ".gif", ".jpg"];
    for path in paths {
        // TODO: Only include "real" media files
        let file = path.unwrap();
        if file.metadata().unwrap().is_file() {
            // Only proceed for real files
            let name = file.file_name().to_str().unwrap().to_string();
            
            if helpers::is_ext_allowed(name.clone()) {
                let url = format!("{}/uploads/{}", blog_context.url, name);
                media_contents.push(url)
            }
        }
    }
    let mut is_token_present = false;
    match token {
        Some(inner) => is_token_present = blog_context.token == inner,
        None => {}
    }
    Template::render(
        "media",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            meta: blog_context.meta.clone(),
            url: blog_context.url.to_owned(),
            token: blog_context.token.to_owned(),
            is_token_present: is_token_present,
            media_contents: media_contents
        },
    )
}


#[get("/delete/<file>?<token>")]
fn media_delete(blog_context: &State<structs::blog::Blog>, file: String, token: String) -> Redirect {
    let path = Path::new("./uploads").join(file);
    if token != blog_context.token {
        return Redirect::to("/");
    }
    if path.exists() {
        let success = std::fs::remove_file(path);
        if success.is_err() {
            //TODO: handle error
            return Redirect::to("/");
        }
    }
    return Redirect::to(format!("/media?token={}", token));
}


#[post("/post/<_id>/<_slug>?<token>", data="<edit_input>")]
fn edit_post(blog_context: &State<structs::blog::Blog>, _id: String, _slug: String, token: String, edit_input: Form<EditInput>) -> Redirect {
    // Check token
    if token != blog_context.token {
        return Redirect::to("/");
    }
    let path = edit_input.path.to_owned();
    let submit = edit_input.submit.to_owned();
    if submit == "delete" {
        let success = fs::remove_file(path);
        if success.is_err() {
            // TODO: error handling
            return Redirect::to(format!("/"));
        }
        return Redirect::to(format!("/?token={}", blog_context.token));
    }
    // TODO: Check path
    let raw_content = edit_input.raw_content.to_owned();
    let return_to = edit_input.return_to.to_owned();
    let write_success = fs::write(path, raw_content);
    //print!("{:?}", edit_input.file.unwrap());
    if write_success.is_err() {
        // TODO: error handling
        return Redirect::to(format!("/"));
    }
    return Redirect::to(return_to);
}

#[get("/post/<id>/<_slug>?<token>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String, _slug: String, token: Option<String>) -> Template {
    let is_edit_mode;
    match token {
        Some(inner)   => is_edit_mode = blog_context.token == inner,
        None          => is_edit_mode = false,
    }

    let post: Post;
    let posts = helpers::get_posts();

    let all_pages: Vec<Post> = posts.iter().filter(|p| p.is_page).cloned().collect();
    if id  != "new" || !is_edit_mode {
        post = posts.into_iter().find(|d| d.id == id).unwrap(); 
    } else {
        let file_name =  Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let full_path = format!("{}{}","./posts/", file_name);
        let id = format!(
            "{:x}",
            md5::compute(full_path.clone().as_bytes())
        );

        post = Post {
            id: id,
            slug:  String::from("new"),
            date: 1,
            title:  String::from("new"),
            content:  String::from(""),
            is_page: false,
            tags: Vec::new(),
            hide_from_robots: false,
            image: String::from(""),
            path: full_path,
            raw_content: String::from(
            "Lorem Ipsum\n\
            1970-01-01 00:00\n\
            :post\n\
            # Lorem Ipsum\n\
            ")
        }
    }
 

    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            p: post.clone(),
            meta: blog_context.meta.clone(),
            url: blog_context.url.to_owned(),
            html: post.html(),
            all_pages: all_pages,
            is_edit_mode: is_edit_mode,
            token: blog_context.token.to_owned()
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
   

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
        .mount("/uploads", FileServer::from("./uploads"))
        .mount("/", routes![index, post, edit_post, feed_url, robots_txt, media_upload, media, media_delete])
        .manage(blog_context)
}
