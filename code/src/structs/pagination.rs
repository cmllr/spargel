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

use crate::{
    helpers,
    structs::post::{self, Post},
    structs::blog::Blog
};
use rocket::State;
use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Readable, Writable)]
pub struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
    pub item_count: usize,
    pub cur_page: usize,
    pub all_pages: usize,
    pub items: Vec<post::Post>,
    pub pages: Vec<post::Post>,
}



impl Pagination {
    // Get all posts from the index pickle, if available
    pub fn get_posts(blog_context: &State<Blog>) -> Pagination {
        let slug_value: String = String::from("index");
        let is_pickled = Pagination::is_pickled(slug_value.clone(), 1);
        return match is_pickled {
            true => {
                Pagination::unpickle(slug_value, 1)
            },
            false => {
                Pagination::get(blog_context, 1,Some(slug_value))
            }
        };
    }
    pub fn get_post(posts: Vec<Post>, id: String) -> Option<Post> {
        return posts.into_iter().find(|d| d.id == id);
    }
    pub fn throw_away_all_pickles() {
        let path = Path::new("./cache");
        let files = fs::read_dir(path);

        match files {
            Ok(inner) => {
                let entries = inner.collect::<Result<Vec<_>, _>>().unwrap();

                for entry in entries {
                    if String::from(entry.file_name().to_str().unwrap()).ends_with(".bin") {
                        let entry_path = path.join(entry.file_name());
                        let _ = fs::remove_file(entry_path);
                    }
                }
            }
            Err(_) => {}
        }
    }
    fn is_pickled(page_slug: String, page: usize) -> bool {
        let page: usize = page;
        if page_slug.len() > 0 {
            let path = Path::new("./cache").join(format!("{}_{}.bin", page_slug, page));
            return path.exists();
        }
        return false;
    }
    fn pickle(result: Pagination, page_slug: String) {
        let page: usize = result.cur_page;
        if page_slug.len() > 0 {
            // Only continue if there is a slug provided
            let path = Path::new("./cache").join(format!("{}_{}.bin", page_slug, page));
            let bytes = result.write_to_vec().unwrap();
            let _ = fs::write(path, bytes);
        }
    }
    fn unpickle(page_slug: String, page: usize) -> Pagination {
        // Only continue if there is a slug provided
        let path = Path::new("./cache").join(format!("{}_{}.bin", page_slug, page));
        let mut f = File::open(path.clone()).expect("no file found");
        let metadata = fs::metadata(path.to_owned().clone()).expect("Whoopsie");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        let deserialized: Pagination = Pagination::read_from_buffer(&buffer).unwrap();
        return deserialized;
    }
    pub fn get(blog_context: &State<Blog>, current_site: usize, tag: Option<String>) -> Pagination {
        // If the named page with the current page was already pickled -> remove the unpickled one instead of querying again

        let slug_value: String = tag.clone().unwrap_or(String::from("index"));
        if Pagination::is_pickled(slug_value.clone(), current_site) {
            let got = Pagination::unpickle(slug_value, current_site);
            return got;
        }

        let mut posts = helpers::get_posts(blog_context);

        let pages: Vec<Post> = posts.iter().filter(|p| p.is_page).cloned().collect();
        posts = posts
            .iter()
            .filter(|p: &&Post| !p.is_page)
            .cloned()
            .collect();
        let total_items_count: usize = posts.len();

        match tag {
            Some(inner) => {
                if inner.len() > 0 && inner != "index" {
                    print!("FILTER={}", inner);
                    posts = posts
                        .iter()
                        .filter(|p| p.tags.contains(&inner))
                        .cloned()
                        .collect();
                }
            }
            None => {}
        }

        let all_items_count = posts.len();
        print!("ITEMS={}, TOTAL={}", all_items_count, total_items_count);

        let post_per_page: usize = 8;

        let all_pages = total_items_count / post_per_page;
        let offset = total_items_count - (all_pages) * post_per_page;
        let incomplete_pages_count: usize = if offset != 0 { 1 } else { 0 };
        let all_pages_count_with_incomplete = all_pages + incomplete_pages_count;
        let has_prev = current_site > 1;
        let has_next = current_site < all_pages_count_with_incomplete;

        let start_index: usize = (current_site - 1) * post_per_page;

        let mut paginated_items: Vec<post::Post> = Vec::new();

        for n in start_index..start_index + post_per_page {
            if n < all_items_count {
                let got: Option<&post::Post> = posts.get(n);
                if got.is_some() {
                    print!("{} index", n);
                    let item: &post::Post = &posts[n];
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
            items: paginated_items,
            pages: pages,
        };
        if slug_value.len() > 0 {
            Pagination::pickle(result.clone(), slug_value);
        }
        return result;
    }
}
