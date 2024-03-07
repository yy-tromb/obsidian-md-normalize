use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

const TARGET_PICTURE_EXTENTIONS: [&str; 8] =
    ["jpeg", "jpg", "png", "gif", "bmp", "webp", "tiff", "apng"];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_md_paths = if args.len() > 1 {
        &args[1..]
    } else {
        panic!(
            "{}",
            "
---------------------------
target file is NOT given.
please use obsidian-md-normalize \"path_to_md\" \"path_to_md\" ...
---------------------------
            "
            .red()
        );
    };
    println!("{:?}", target_md_paths);
    for target_md_path in target_md_paths {
        let mut target_md = match File::open(target_md_path) {
            Ok(file) => file,
            Err(ref err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    panic!(
                        "{}\n{:?}",
                        format!("file: [{target_md_path}] is not found.").red(),
                        err
                    );
                } else {
                    panic!(
                        "{}\n{:?}",
                        format!(
                            "UNKNOWN PROBLEM was happend on opening the file: [{target_md_path}]"
                        )
                        .red(),
                        err
                    );
                }
            }
        };
        let mut contents = String::new();
        if let Err(err) = target_md.read_to_string(&mut contents) {
            panic!(
                "{}\n{:?}",
                format!("file: [{target_md_path}] COULD NOT be read.").red(),
                err
            );
        }
        println!("{}", contents);
        // start feature part
        let contents = fix_new_line(&contents);
        let contents = fix_picture_embeded(&contents);
        // end feature part
        let mut output = match File::create(make_new_file_name(&target_md_path)) {
            Ok(file) => file,
            Err(ref err) => {
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    panic!(
                        "{}\n{:?}",
                        format!("file: [{target_md_path}] has already exist.").red(),
                        err
                    )
                } else {
                    panic!(
                        "{}\n{:?}",
                        format!(
                            "UNKNOWN PROBLEM was happend on creating the file: [{target_md_path}]"
                        )
                        .red(),
                        err
                    );
                }
            }
        };
        match write!(output, "{contents}") {
            Ok(_) => (),
            Err(err) => panic!(
                "{}\n{:?}",
                format!("Problem was happend on writing to the file: [{target_md_path}]").red(),
                err
            ),
        };
        match output.flush() {
            Ok(_) => println!(
                "{}",
                format!("[{target_md_path}] is sucessflly processed.").green()
            ),
            Err(err) => panic!(
                "{}\n{:?}",
                format!("Problem was happend on writing to the file: [{target_md_path}]").red(),
                err
            ),
        }
    }
}

fn make_new_file_name(old_path: &str) -> PathBuf {
    let path = Path::new(old_path);
    let mut new_path = PathBuf::new();
    if let Some(parent) = path.parent() {
        new_path.push(parent);
    };
    if let (Some(stem), Some(extention)) = (path.file_stem(), path.extension()) {
        new_path.push(format!(
            "{}_formatted.{}",
            stem.to_string_lossy(),
            extention.to_string_lossy()
        ));
    };
    new_path
}

// feature fns

fn fix_new_line(contents: &str) -> String {
    let regex = "[\r\n|\n|\r]";
    let regex = Regex::new(regex).unwrap();
    regex.replace_all(contents, "  $0").to_string()
}

fn fix_picture_embeded(contents: &str) -> String {
    let regex = format!(r"!\[\[(.+\.(?:{}))\]\]", format_picture_extentions(None));
    let regex = Regex::new(&regex).unwrap();
    regex.replace_all(contents, "![]($1)").to_string()
}

fn format_picture_extentions(additional_extentions: Option<&[&str]>) -> String {
    let mut picture_extentions = Vec::from(TARGET_PICTURE_EXTENTIONS);
    if let Some(extentions) = additional_extentions {
        picture_extentions.extend(extentions);
    }
    picture_extentions.join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_picture_embeded() {
        let contents = "\
# Hoge
![](normal picture link)
![[obsidian picture link.jpg]]"
            .to_string();
        let replaced = fix_picture_embeded(&contents);
        assert_eq!(
            replaced,
            "\
# Hoge
![](normal picture link)
![](obsidian picture link.jpg)"
        );
    }

    #[test]
    fn test_fix_new_line() {
        let contents = r"# Hoge
![](normal picture link)
![[obsidian picture link.jpg]]"
            .to_string();
        let replaced = fix_new_line(&contents);
        assert_eq!(
            replaced,
            "\
# Hoge  
![](normal picture link)  
![[obsidian picture link.jpg]]"
        );
    }

    #[test]
    fn my_regex_study() {
        let regex = Regex::new(r"!\[\[(.+\.(?:ab|a))\]\]").unwrap();
        assert_eq!(regex.replace_all("![[a b c.ab]]", "($1)"), "(a b c.ab)");
    }
}
