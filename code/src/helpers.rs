use std::io;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::UNIX_EPOCH;
use crate::structs::post::Post;

extern crate slugify;
use slugify::slugify;

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> Result<Vec<fs::DirEntry>, io::Error> {
    let mut paths = fs::read_dir(path)?
            .collect::<Result<Vec<_>, _>>()?;
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(paths)
}


pub fn get_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();

    let post_folder = "./posts";
    let paths = read_dir_sorted(post_folder).unwrap();


    for year_path in paths {
        let year_path_unwrapped = year_path;
        let full_year_path = Path::new(post_folder).join(year_path_unwrapped.file_name());

        let month_paths = read_dir_sorted(full_year_path.clone()).unwrap();

        for month_path in month_paths {
            let month_path_unwrapped = month_path;
            let full_month_path = full_year_path.join(month_path_unwrapped.file_name());
            let found_folders = read_dir_sorted(full_month_path.clone()).unwrap();

            for folder in found_folders {
                if folder.metadata().unwrap().is_dir() {

                    let full_path = Path::new(&full_month_path.clone()).join(folder.file_name());
                    let post_files = read_dir_sorted(full_path.clone()).unwrap();

                    for file in post_files {
                        let full_file_path = Path::new(&full_path.clone()).join(file.file_name());
                        let id = format!(
                            "{:x}",
                            md5::compute(full_file_path.clone().into_os_string().as_bytes())
                        );
                        let file_created = fs::metadata(&full_path).unwrap().created();        
                        let system_time = file_created.unwrap();        
                        let file_created_timestamp = system_time
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();      

                        let raw_content = fs::read_to_string(full_file_path.clone()).unwrap();
                        let title = raw_content.lines().next().unwrap().to_string();
                        let content = raw_content.lines().skip(1).next().unwrap().to_string();
                        let slug = slugify!(&title.clone().as_str());
                       
                        let post = Post {
                            id: id,
                            title: title,
                            date: file_created_timestamp,
                            content: content,
                            slug: slug
                        };
                        posts.push(post); 
                    }
                }
            }
        }
    }
    posts.reverse();
    return posts;
}