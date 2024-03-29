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

use crate::feed;
use crate::structs;
use crate::helpers;
use crate::structs::post::Post;


#[get("/feed")]
pub fn feed_url(blog_context: &State<structs::blog::Blog>) -> feed::FeedResponse {
    let posts = helpers::get_posts(blog_context);
    let all_posts: Vec<Post> = posts.iter().filter(|p| !p.is_page).cloned().collect();
   
    let content = feed::get_feed(blog_context.inner().clone(), all_posts);
    content
}