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
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

extern crate slugify;
use chrono::format;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
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
            let is_page: bool = raw_content.lines().skip(2).next().unwrap().to_string() == "page";
            for line in raw_content.lines().skip(3) {
                content.push_str(format!("{}\n", line).as_str());
            }
            let slug = slugify!(&title.clone().as_str());

            let post = Post {
                id: id,
                title: title,
                date: parsed_date.timestamp(),
                content: content,
                slug: slug,
                is_page: is_page
            };
            posts.push(post);
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    return posts;
}
