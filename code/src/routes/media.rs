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

use std::{fs, path::Path};

use image::GenericImageView;
use rocket::{data::ToByteUnit, response::Redirect, State};
use rocket::Data;
use rocket_dyn_templates::{context, Template};

use crate::{helpers, structs};

#[post("/media?<token>&<size>&<name>", data = "<paste>")]
pub async fn media_upload(blog_context: &State<structs::blog::Blog>, token: String, size: u64, name: String, paste: Data<'_>) -> Redirect {
    // Check token
    if token != blog_context.token {
        return Redirect::to("/");
    }
    // TODO: That protection ist just RIDICULOUSLY BAD

    if helpers::is_ext_allowed(name.clone())
    {
        let path = Path::new("./uploads").join(name.clone());
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
        // Create a thumbnail on upload
        let thumb_path = Path::new("./uploads").join(format!("thumb_{}", name));
        let _ = img.thumbnail(64, 64).save(thumb_path);
    } else {
        return Redirect::to("/");
    }

    // TODO: Scale down images wider than 1024
    return Redirect::to("/media");
}

#[get("/media?<token>")]
pub fn media(blog_context: &State<structs::blog::Blog>, token: Option<String>) -> Template {
    let mut media_contents: Vec<String> = Vec::new();
    let paths = fs::read_dir("./uploads").unwrap();
    // TODO: make global-ish
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
        format!("{}/media", blog_context.theme),
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
pub fn media_delete(blog_context: &State<structs::blog::Blog>, file: String, token: String) -> Redirect {
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