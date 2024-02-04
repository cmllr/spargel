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


use rss::ChannelBuilder;
use rss::ItemBuilder;
use rss::Item;


use std::io::Cursor;

use rocket::{response, Response};
use rocket::request::Request;
use rocket::response::Responder;

use crate::structs::post::Post;
use crate::structs::blog::Blog;


pub struct FeedResponse {
    pub content: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for FeedResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .raw_header("content-type", "application/xml; charset=UTF-8")
            .raw_header("content-disposition","inline")
            .sized_body(self.content.len(), Cursor::new(self.content))
            .ok()
    }
}
// TODO: Paginate?
pub fn get_feed(context: Blog, items: Vec<Post>) -> FeedResponse{
    let mut channel = ChannelBuilder::default()
        .title(context.title)
        .description(context.sub_title)
        .build();
    let mut raw_items: Vec<Item> = Vec::new();
    for post in items {
        let post_content = post.clone().html();
        let p = ItemBuilder::default()
        .title(post.title)
        .content(Some(post_content))
        .link(format!("{}/post/{}/{}", context.url, post.id, post.slug))
        .build();
        raw_items.push(p);
    }
    channel.set_items(raw_items);
    return FeedResponse{
        content: channel.to_string()
    }
}