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

use std::{env, fs, os::unix::ffi::OsStrExt, path::Path};

use rand::distributions::{Alphanumeric, DistString};
use rocket::{response::Redirect, State};
use rocket_dyn_templates::{context, Template};
use rocket::form::Form;

use crate::structs::{self, pagination::Pagination, post::Post};

use crate::routes::util;

#[derive(FromForm)]
struct EditInput {
    path: String,
    raw_content: String,
    return_to: String,
    submit: String,
}

#[post("/post/<_id>/<_slug>?<token>", data="<edit_input>")]
pub fn edit_post(blog_context: &State<structs::blog::Blog>, _id: String, _slug: String, token: String, edit_input: Form<EditInput>) -> Redirect {
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

    Pagination::throw_away_all_pickles();
    
    return Redirect::to(return_to);
}

#[get("/post/<id>/<_slug>?<token>")]
pub fn post(blog_context: &State<structs::blog::Blog>, id: String, _slug: String, token: Option<String>) -> Template {
    let is_edit_mode;
    match token {
        Some(inner)   => is_edit_mode = blog_context.token == inner,
        None          => is_edit_mode = false,
    }

    let post: Post;
    let pagination = Pagination::get_posts(blog_context);

    let all_pages: Vec<Post> = pagination.pages.clone();
    if id  != "new" || !is_edit_mode {
        let mut post_candidate  = Pagination::get_post(pagination.items, id.clone());
        if !post_candidate.is_some() {
            post_candidate = Pagination::get_post(all_pages.clone(), id.clone());
        }
        if !post_candidate.is_some() {
            return crate::routes::util::error(blog_context, 404);
        }
        post = post_candidate.unwrap();
       
    } else {
        // FIXME:The return_to leads to a 404 (serialziaton issue?)
        let file_name =  Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let full_path = Path::new(&env::current_dir().unwrap()).join("posts").join(file_name);
        let id = format!(
            "{:x}",
            md5::compute(full_path.clone().into_os_string().as_bytes())
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
            path: full_path.into_os_string().into_string().unwrap(),
            raw_content: String::from(
            "Lorem Ipsum\n\
            1970-01-01 00:00\n\
            :post\n\
            # Lorem Ipsum\n\
            ")
        }
    }
 

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
