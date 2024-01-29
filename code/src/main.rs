#[macro_use]
extern crate rocket;
extern crate chrono;

use rocket::http::RawStr;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use std::borrow::Borrow;
use std::fmt::format;
use std::{clone, fs};
use std::fs::File;
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::UNIX_EPOCH;
use structs::post::Post;
use chrono::DateTime;

use chrono::offset::Utc;
use chrono::Local;
mod structs;

#[get("/")]
fn index(blog_context: &State<structs::blog::Blog>) -> Template {
    Template::render(
        "index",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            posts: get_posts()
        },
    )
}

#[get("/<id>")]
fn post(blog_context: &State<structs::blog::Blog>, id: String) -> Template {
    let posts = get_posts();
    let post = posts.into_iter().find(|d| d.id == id).unwrap();
    // TODO: 404
    Template::render(
        "post",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            post: post
        },
    )
}



fn get_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let post_folder = "./posts";
    let paths = fs::read_dir(post_folder).unwrap();



    for year_path in paths {
        let year_path_unwrapped = year_path.unwrap();
        let full_year_path = Path::new(post_folder).join(year_path_unwrapped.file_name());

        let month_paths = fs::read_dir(full_year_path.clone()).unwrap();

        for month_path in month_paths {
            let month_path_unwrapped = month_path.unwrap();
            let full_month_path = full_year_path.join(month_path_unwrapped.file_name());
            let found_files = fs::read_dir(full_month_path.clone()).unwrap();
            for file in found_files {
                let file_unwrapped = file.unwrap();
                let full_path = Path::new(&full_month_path.clone()).join(file_unwrapped.file_name());


                let file_created = fs::metadata(&full_path).unwrap().created();        
                let system_time = file_created.unwrap();        
                let file_created_timestamp = system_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();        
                let file_created_timestamp_date_obj: DateTime<Utc> = system_time.into();
                let day_group = format!("{}", file_created_timestamp_date_obj.format("%d"));
                let post = Post {
                    id: format!(
                        "{:x}",
                        md5::compute(file_unwrapped.file_name().as_bytes())
                    ),
                    title: "".to_string(),
                    date: file_created_timestamp,
                    content: fs::read_to_string(full_path).unwrap(),
                    month_group: format!("{:?}{:?}", year_path_unwrapped, month_path_unwrapped),
                    day_group: day_group
                };
                posts.push(post);
            }
        }
    }

    return posts;
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
