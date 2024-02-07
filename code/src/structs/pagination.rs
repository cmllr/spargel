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

use std::{fs::{self, File}, io::Read, path::Path};
use speedy::{Readable, Writable};
use serde::{Deserialize, Serialize};
use crate::structs::post;

#[derive(Serialize, Deserialize, Clone, PartialEq, Readable, Writable)]
pub struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
    pub item_count: usize,
    pub cur_page: usize,
    pub all_pages: usize,
    pub items: Vec<post::Post>
}

impl Pagination{
    fn is_pickled(page_slug: Option<String>, page: usize) -> bool {
        let page: usize = page;
        let slug: String = page_slug.unwrap();
        if slug.len() > 0 {
            let path = Path::new("./cache").join(format!("{}_{}.bin", slug, page));
            return path.exists();
        }
        return false;
    }
    fn pickle(result: Pagination, page_slug: Option<String>){
        let slug: String = page_slug.unwrap();
        let page: usize = result.cur_page;
        if slug.len() > 0 {
            // Only continue if there is a slug provided
            let path = Path::new("./cache").join(format!("{}_{}.bin", slug, page));
            let bytes = result.write_to_vec().unwrap();
            let _ = fs::write(path, bytes);
        }
    }
    fn unpickle(page_slug: Option<String>, page: usize) -> Pagination{
        // Only continue if there is a slug provided
        let path = Path::new("./cache").join(format!("{}_{}.bin", page_slug.unwrap(), page));
        let mut f = File::open(path.clone()).expect("no file found");
        let metadata = fs::metadata(path.to_owned().clone()).expect("Whoopsie");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        let deserialized: Pagination =
        Pagination::read_from_buffer( &buffer ).unwrap();
        return deserialized
    }
    pub fn get(total_items_count: usize, current_site: usize, items: Vec<post::Post>, page_slug: Option<String>) -> Pagination {
        // If the named page with the current page was already pickled -> remove the unpickled one instead of querying again
        if Pagination::is_pickled(page_slug.clone(), current_site) {
            let got =  Pagination::unpickle(page_slug.clone(), current_site);
            return got;
        }

        let post_per_page: usize = 8;
    
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
        let result: Pagination = Pagination {
            has_next: has_next,
            has_prev: has_prev,
            item_count: all_pages,
            cur_page: current_site,
            all_pages: all_pages_count_with_incomplete,
            items: paginated_items
        };
        if page_slug.to_owned().clone().is_some() {
            Pagination::pickle(result.clone(), page_slug);
        }
        return result;
    }
}