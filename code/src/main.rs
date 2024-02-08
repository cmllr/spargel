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

use std::{fs::File, io::Read};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use structs::pagination::Pagination;

mod feed;
mod helpers;
mod structs;
mod routes;

#[launch]
fn rocket() -> _ {
    let mut file = File::open("blog.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let blog_context: structs::blog::Blog = serde_json::from_str(&buff).unwrap();
   
    Pagination::throw_away_all_pickles();

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
        .mount("/uploads", FileServer::from("./uploads"))
        .mount("/", routes![routes::index::index, routes::post::post, routes::post::edit_post, routes::rss::feed_url, routes::util::robots_txt, routes::media::media_upload, routes::media::media, routes::media::media_delete])
        .manage(blog_context)
}
