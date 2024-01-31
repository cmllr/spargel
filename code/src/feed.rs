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
        let p = ItemBuilder::default()
        .title(post.title)
        .content(post.content)
        .link(format!("{}/post/{}/{}", context.url, post.id, post.slug))
        .build();
        raw_items.push(p);
    }
    channel.set_items(raw_items);
    return FeedResponse{
        content: channel.to_string()
    }
}