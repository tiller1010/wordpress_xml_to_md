use std::fs::File;
use std::io::Write;
use std::path::Path;
use html2md::parse_html;

mod read_lines;
mod format_date;

fn main() {
    if let Ok(lines) = read_lines::read_lines("./source.xml") {
        let mut is_under_post = false;
        for line in lines {
            if let Ok(ip) = line {
                if ip.contains("<item>") && !is_under_post {
                    is_under_post = true;
                } else if ip.contains("<title>") && is_under_post {
                    let originalTitle = ip.clone();
                    let mut title = originalTitle.replace("<title>", "");
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

                    // get the date from the pubDate tag
                    let mut date = String::new();
                    let mut date_flag = false;
                    let mut date_is_under_title = false;
                    if let Ok(lines) = read_lines::read_lines("./source.xml") {
                        for line in lines {
                            if let Ok(ip) = line {
                                if ip.contains(&originalTitle) {
                                    date_is_under_title = true;
                                    continue;
                                }
                                if date_is_under_title {
                                    if ip.contains("<pubDate>") {
                                        date_flag = true;
                                    }
                                    if date_flag {
                                        date = date + &ip;
                                    }
                                    if ip.contains("</pubDate>") {
                                        date_flag = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if date != "" {
                        date = date.replace("<pubDate>", "");
                        date = date.replace("</pubDate>", "");
                        date = date.trim().to_string();

                        let formatted_date = format_date::format_date(&date).unwrap();
                        date = formatted_date;
                    } else {
                        date = "2020-01-01 00:00:00 UTC".to_string();
                    }

                    // get the tags from <category domain="post_tag">
                    let mut tags = String::new();
                    let mut tags_flag = false;
                    let mut tags_is_under_title = false;
                    if let Ok(lines) = read_lines::read_lines("./source.xml") {
                        for line in lines {
                            if let Ok(ip) = line {
                                if ip.contains(&originalTitle) {
                                    tags_is_under_title = true;
                                    continue;
                                }
                                if tags_is_under_title {
                                    if ip.contains("<category domain=\"post_tag\"") {
                                        tags_flag = true;
                                    }
                                    if tags_flag {
                                        // get the nicename attribute
                                        let tag = ip.split("nicename=\"").collect::<Vec<&str>>()[1].split("\"").collect::<Vec<&str>>()[0].to_string();
                                        tags = tags + &tag;
                                    }
                                    if ip.contains("</category>") {
                                        tags_flag = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    // get the content from the next <content:encoded> tag
                    let mut content = String::new();
                    let mut content_flag = false;
                    let mut content_is_under_title = false;
                    if let Ok(lines) = read_lines::read_lines("./source.xml") {
                        for line in lines {
                            if let Ok(ip) = line {
                                if ip.contains(&originalTitle) {
                                    content_is_under_title = true;
                                    continue;
                                }
                                if content_is_under_title {
                                    if ip.contains("<content:encoded>") {
                                        content_flag = true;
                                    }
                                    if content_flag {
                                        content = content + "\n" + &ip;
                                    }
                                    if ip.contains("</content:encoded>") {
                                        content_flag = false;
                                        break;
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

                    // combine created date, tags, heading, and content
                    let markdown_content = "created: ".to_owned() + &date + "\ntags: [" + &tags + "]\n\n" + &markdown_heading + "\n\n" + &content;

                    match file.write_all(markdown_content.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }

                }
            }
        }
    }
}

