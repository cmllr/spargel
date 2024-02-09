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

use crate::{helpers, structs::{self, blog, pagination::Pagination, post::Post}};

pub fn error(blog_context: &State<structs::blog::Blog>, status: usize) -> Template {
    let pagination = Pagination::get_posts(blog_context);
    // TODO: Where to get posts? 
    
    let all_pages: Vec<Post> = pagination.pages;
    Template::render(
        "error",
        context! {
            title: blog_context.title.to_owned(),
            sub_title:  blog_context.sub_title.to_owned(),
            meta: blog_context.meta.clone(),
            url: blog_context.url.to_owned(),
            all_pages: all_pages,
            token: blog_context.token.to_owned(),
            status: status
        },
    )
}


#[get("/robots.txt")] 
pub fn robots_txt(blog_context: &State<structs::blog::Blog>) -> String {
    let mut robots_content = String::from("User-Agent: *\n");
    let posts = helpers::get_posts(blog_context);
    let hidden_posts: Vec<Post> = posts.iter().filter(|p| p.hide_from_robots).cloned().collect();

    for post in hidden_posts {
        robots_content.push_str(format!("Disallow: {}\n", post.url()).as_str());
    }
    
    return robots_content;
}