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

use rocket::State;
use rocket_dyn_templates::{context, Template};

use crate::structs;



#[get("/?<tag>&<page>&<token>")]
pub fn index(blog_context: &State<structs::blog::Blog>, tag: Option<String>, page: Option<usize>, token: Option<String>) -> Template {

    let current_site: usize = page.unwrap_or(1);
    let pagination =
        structs::pagination::Pagination::get(blog_context, current_site, tag);
    let mut is_edit_mode = false;
    if token.is_some() {
        if token.unwrap() == blog_context.token {
            is_edit_mode = true;
        }
    }

  
    let all_pages = pagination.pages.clone();
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