use std::borrow::Borrow;

use serde::{Deserialize, Serialize};


use crate::structs::post;

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
    pub item_count: usize,
    pub cur_page: usize,
    pub all_pages: usize,
    pub items: Vec<post::Post>
}

impl Pagination{
    pub fn get(total_items_count: usize, current_site: usize, items: Vec<post::Post>) -> Pagination {

        let post_per_page: usize = 2;
    
        let all_items_count = items.len();
        let all_pages = total_items_count / post_per_page;
        let offset = total_items_count - (all_pages)*post_per_page;
        let incomplete_pages_count: usize = if offset != 0 { 1 } else { 0 };
        let all_pages_count_with_incomplete = all_pages + incomplete_pages_count;
        let has_prev = current_site > 1;
        let has_next = current_site < all_pages_count_with_incomplete;

        let start_index: usize = (current_site -1) * post_per_page;

        let mut paginated_items: Vec<post::Post> = Vec::new();



        for n in start_index..start_index + post_per_page {
            if n < all_items_count {
                let got: Option<&post::Post> = items.get(n);
                if got.is_some() {
                    print!("{} index", n);
                    let item: &post::Post = &items[n];
                    paginated_items.push(item.clone());
                }
            }
        }
        return Pagination {
            has_next: has_next,
            has_prev: has_prev,
            item_count: all_pages,
            cur_page: current_site,
            all_pages: all_pages_count_with_incomplete,
            items: paginated_items
        }
    }
}