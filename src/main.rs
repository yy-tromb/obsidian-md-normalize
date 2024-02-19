use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_md_paths = if args.len() > 1 {
        &args[1..]
    } else {
        panic!(
            r#"
---------------------------
target file is NOT given.
please use obsidian-md-normalize "path_to_md" "path_to_md" ...
---------------------------
            "#
        );
    };
    println!("{:?}", target_md_paths);
    for target_md_path in target_md_paths {
        let mut target_md = match File::open(target_md_path) {
            Ok(file) => file,
            Err(ref err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    panic!("file: [{target_md_path}] is not found.\n{:?}", err);
                } else {
                    panic!(
                        "UNKNOWN PROBLEM was happend on opening the file: [{target_md_path}]\n{:?}",
                        err
                    );
                }
            }
        };
        let mut contents = String::new();
        if let Err(err) = target_md.read_to_string(&mut contents) {
            panic!("file: [{target_md_path}] COULD NOT be read.\n{:?}", err);
        }
        println!("{}", contents);
    }
}
