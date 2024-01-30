use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination<T> {
    pub has_next: bool,
    pub has_prev: bool,
    pub item_count: u32,
    pub cur_page: u32,
    pub all_pages: u32,
    pub items: Vec<T>
}

impl<T> Pagination <T>{
    pub fn get(total_items_count: u32, current_site: u32, items: Vec<T>) -> Pagination<T>{

        let post_per_page: u32 = 2;
    
    
        let all_pages = total_items_count / post_per_page;
        let offset = total_items_count - (all_pages)*post_per_page;
        let incomplete_pages_count = if offset != 0 { 1 } else { 0 };
        let all_pages_count_with_incomplete = all_pages + incomplete_pages_count;
        let has_prev = current_site > 1;
        let has_next = current_site < all_pages_count_with_incomplete;

        let start_index: usize = ((current_site -1) * post_per_page) as usize;
        return Pagination {
            has_next: has_next,
            has_prev: has_prev,
            item_count: all_pages,
            cur_page: current_site,
            all_pages: all_pages_count_with_incomplete,
            items: [] // TODO: Slice 
        }
    }
}