use std::collections::BTreeMap;
use indexmap::IndexMap;


use std::io;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::UNIX_EPOCH;
use chrono::DateTime;

use crate::structs::post::Post;

use chrono::offset::Utc;

pub fn group_posts(raw_posts: Vec<Post>) -> IndexMap<String, IndexMap<String, Vec<Post>>> {
    // 202401 -> [01: [Post, Post, Post], 02: [Post, Post]]
    let mut results: IndexMap<String, IndexMap<String, Vec<Post>>> = IndexMap::new();

    for post in raw_posts {
        let year_group = post.year_month_group.clone();
        let day_group: String = post.day_group.clone();
        // Check if the year is present within the hashmap
        if !results.contains_key(&year_group){
            
            let days_hashmap: IndexMap<String, Vec<Post>> = IndexMap::new();
            results.insert(year_group, days_hashmap);
        }
        
        let year_group_clone = post.year_month_group.clone();
        // Get the day: Posts map
        let entry = results.get(&year_group_clone).unwrap();

        if !entry.contains_key(&post.day_group){
            let mut posts_vec: Vec<Post> = Vec::new();
            posts_vec.push(post);
            results.get_mut(&year_group_clone).unwrap().insert(day_group, posts_vec);
        } else {
            results.get_mut(&year_group_clone).unwrap().get_mut(&day_group).unwrap().push(post);
        }
    }
    return results;
}

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> Result<Vec<fs::DirEntry>, io::Error> {
    let mut paths = fs::read_dir(path)?
            .collect::<Result<Vec<_>, _>>()?;
    paths.sort_by_key(|de| de.metadata().unwrap().created().unwrap());
    paths.reverse();
    Ok(paths)
}


pub fn get_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let post_folder = "./posts";
    let paths = fs::read_dir(post_folder).unwrap();



    for year_path in paths {
        let year_path_unwrapped = year_path.unwrap();
        let full_year_path = Path::new(post_folder).join(year_path_unwrapped.file_name());

        let month_paths = read_dir_sorted(full_year_path.clone()).unwrap();

        for month_path in month_paths {
            let month_path_unwrapped = month_path;
            let full_month_path = full_year_path.join(month_path_unwrapped.file_name());
            let found_files = fs::read_dir(full_month_path.clone()).unwrap();

            for file in found_files {
                let file_unwrapped = file.unwrap();
                let full_path = Path::new(&full_month_path.clone()).join(file_unwrapped.file_name());


                let file_created = fs::metadata(&full_path).unwrap().created();        
                let system_time = file_created.unwrap();        
                let file_created_timestamp = system_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();        
                let file_created_timestamp_date_obj: DateTime<Utc> = system_time.into();
                let day_group = format!("{}", file_created_timestamp_date_obj.format("%d"));
                let post = Post {
                    id: format!(
                        "{:x}",
                        md5::compute(file_unwrapped.file_name().as_bytes())
                    ),
                    title: "".to_string(),
                    date: file_created_timestamp,
                    content: fs::read_to_string(full_path).unwrap(),
                    year_month_group: format!("{:?}{:?}", year_path_unwrapped.file_name(), month_path_unwrapped.file_name()),
                    day_group: day_group
                };
                posts.push(post);
            }
        }
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    return posts;
}