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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: i64,
    pub content: String,
    pub slug: String,
    pub is_page: bool,
    pub tags: Vec<String>,
    pub hide_from_robots: bool
}

impl Post {
    pub fn html(self) -> String {
        return markdown::to_html(self.content.as_str());
    }
    pub fn url(self) -> String {
        return String::from(format!("/post/{}/{}", self.id, self.slug));
    }
}