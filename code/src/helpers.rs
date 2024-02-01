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

use crate::structs::post::Post;
use std::borrow::Borrow;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::time::UNIX_EPOCH;

extern crate slugify;
use chrono::format;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use indexmap::map::raw_entry_v1;
use slugify::slugify;

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> Result<Vec<fs::DirEntry>, io::Error> {
    let mut paths = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(paths)
}

pub fn get_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let post_folder = "./posts";
    let post_files = read_dir_sorted(post_folder).unwrap();

    for file in post_files {
        let full_file_path = Path::new(&post_folder).join(file.file_name());
        if file.file_type().unwrap().is_file() {
            let id = format!(
                "{:x}",
                md5::compute(full_file_path.clone().into_os_string().as_bytes())
            );

            let raw_content = fs::read_to_string(full_file_path.clone()).unwrap();
            let title = raw_content.lines().next().unwrap().to_string();
            let raw_date: String = raw_content.lines().skip(1).next().unwrap().to_string();
            let parsed_date: NaiveDateTime =
                NaiveDateTime::parse_from_str(&raw_date, "%Y-%m-%d %H:%M").unwrap();
            let mut content = String::new();
            let raw_tags = raw_content.lines().skip(2).next().unwrap().to_string();
            let raw_tags_vec: Vec<&str> = raw_tags.split(",").collect();
            let mut tags: Vec<String> = Vec::new();
            let mut hide_from_bots: bool = false;
            let mut is_page: bool = false;
            for raw_tag in raw_tags_vec {
                let raw_tag_string = raw_tag.to_string();
                let stripped_tag = raw_tag_string.trim().to_string().replace(":", "");
                if stripped_tag == "hide" {
                    hide_from_bots = true;
                }
                else if stripped_tag == "page" {
                    is_page = true;
                } else if stripped_tag == "post" {
                    /* do nothing as the default is post */
                } else {
                    tags.push(stripped_tag.to_string());
                }
            }


            for line in raw_content.lines().skip(3) {
                content.push_str(format!("{}\n", line).as_str());
            }
            let slug = slugify!(&title.clone().as_str());
            let mut post = Post {
                id: id,
                title: title,
                date: parsed_date.timestamp(),
                content: content,
                slug: slug,
                is_page: is_page,
                tags: tags,
                hide_from_robots: hide_from_bots,
                image: String::new()
            };
            let images: Vec<String> = post.clone().images();
            if images.len() > 0 {
                let image: &String = images.get(0).unwrap();
                post.image = image.to_string();
            }
            posts.push(post);
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    return posts;
}
