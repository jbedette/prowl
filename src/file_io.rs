use crate::shared::application_root_dir;

//thank you rust by example https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub fn read_file(f_name: &str) -> Vec<String> {
    let mut name_vec = Vec::new();
    if let Ok(lines) = read_lines(f_name) {
        for line in lines {
            if let Ok(name) = line {
                name_vec.push(name);
            }
        }
    }
    /*
    for name in name_vec {
        println! {"name:{}",name};
    }
    println!("end");
    */
    name_vec
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // works from whatever directory in project you run it from, instead of just
    // if you run at root.
    let mut file_path = application_root_dir()?;
    file_path = file_path.join(filename);
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
