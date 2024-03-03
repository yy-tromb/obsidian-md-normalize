use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

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
                        "file: [{target_md_path}] is not found.".red(),
                        err
                    );
                } else {
                    panic!(
                        "{}\n{:?}",
                        "UNKNOWN PROBLEM was happend on opening the file: [{target_md_path}]".red(),
                        err
                    );
                }
            }
        };
        let mut contents = String::new();
        if let Err(err) = target_md.read_to_string(&mut contents) {
            panic!(
                "{}\n{:?}",
                "file: [{target_md_path}] COULD NOT be read.".red(),
                err
            );
        }
        println!("{}", contents);
        let contents = fix_picture_embeded(&contents);
    }
}

fn fix_picture_embeded(contents: &str) -> String {
    let regex = format!(r"!\[\[(.+.[{}])\]\]", format_picture_extentions(None));
    println!("{}", regex);
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
        let contents = r"\
# Hoge
![](normal picture link)
![[obsidian picture link.jpg]]"
            .to_string();
        let replaced = fix_picture_embeded(&contents);
        assert_eq!(
            replaced,
            r"\
# Hoge
![](normal picture link)
![](obsidian picture link.jpg)"
        );
    }

    #[test]
    fn my_regex_search() {
        let regex = Regex::new(r"!\[\[(.*)\]\]").unwrap();
        assert_eq!(regex.replace_all("![[a]]", "($1)"), "(a)");
    }
}
