use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./source.xml") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("<title>") {
                    let mut title = ip.replace("<title>", "");
                    title.retain(|c| c.is_alphabetic() || c == ' ');

                    let file_title = title.replace(" ", "_");

                    let path_name = "./output/".to_owned() + &file_title + ".md";
                    let path = Path::new(&path_name);
                    let display = path.display();

                    let mut file = match File::create(&path) {
                        Err(why) => panic!("couldn't create {}: {}", display, why),
                        Ok(file) => file,
                    };

                    let markdown_heading = "# ".to_owned() + &title + "\n";

                    match file.write_all(markdown_heading.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }

                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

