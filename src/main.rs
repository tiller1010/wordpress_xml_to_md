use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use html2md::parse_html;

fn main() {
    if let Ok(lines) = read_lines("./source.xml") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("<title>") {
                    let mut title = ip.replace("<title>", "");
                    title = title.replace("</title>", "");
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

                    // get the content from the next <content:encoded> tag
                    let mut content = String::new();
                    let mut content_flag = false;
                    let mut is_under_title = false;
                    if let Ok(lines) = read_lines("./source.xml") {
                        for line in lines {
                            if let Ok(ip) = line {
                                if ip.contains(&title) {
                                    is_under_title = true;
                                    continue;
                                }
                                if is_under_title {
                                    if ip.contains("<content:encoded>") {
                                        content_flag = true;
                                    }
                                    if content_flag {
                                        content = content + &ip;
                                    }
                                    if ip.contains("</content:encoded>") {
                                        content_flag = false;
                                    }
                                }
                            }
                        }
                    }

                    // remove any wordpress elements
                    content = content.replace("<content:encoded>", "");
                    content = content.replace("</content:encoded>", "");
                    content = content.replace("<![CDATA[", "");
                    content = content.replace("<!-- wp:paragraph -->", "");
                    content = content.replace("<!-- /wp:paragraph -->", "");
                    content = content.replace("<!-- wp:image -->", "");
                    content = content.replace("<!-- /wp:image -->", "");
                    content = content.replace("<!-- wp:heading -->", "");
                    content = content.replace("<!-- /wp:heading -->", "");

                    // convert html to markdown
                    content = parse_html(&content);
                    content = content.replace("]]\\>", "");

                    // combine markdown heading, newline, and content
                    let markdown_content = markdown_heading + "\n\n" + &content;

                    match file.write_all(markdown_content.as_bytes()) {
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

