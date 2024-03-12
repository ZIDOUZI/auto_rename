use std::collections::HashMap;
use glob::glob;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use tree_magic;

static MIME: &str = include_str!("./mimes.json");

#[derive(PartialEq)]
enum Silence {
    No,
    OnlyErrors,
    Yes,
}

fn rename_file(file: &PathBuf, mt: &Value, silence: &Silence) {
    let mut cp = PathBuf::from(file);
    let f = tree_magic::from_filepath(file);
    let ext = &mt[f];
    if ext.is_null() {
        if *silence != Silence::Yes {
            println!("Could not find extension for {:?}", file);
        }
    } else {
        cp.set_extension(ext.as_str().unwrap());
        if file.eq(&cp) {
            if silence == &Silence::No {
                println!("Skipping {:?}", file);
            }
        } else {
            if silence != &Silence::Yes {
                println!("{:?} => {:?}", file, cp);
            }
            fs::rename(file, cp).unwrap_or_else(|_| if silence != &Silence::Yes { println!("could not rename {:#?}", file) });
        }
    };
}

fn walk(path: &str, mt: &Value, silence: &Silence) {
    let p = PathBuf::from(path);
    if !p.exists() && silence != &Silence::Yes {
        println!("{} does not exist!", p.display())
    }

    if p.is_dir() {
        let pb: PathBuf = p.join("*");
        let star = glob(pb.to_str().unwrap()).expect("Failed to create a glob pattern");
        for i in star {
            let f_path = i.unwrap();
            let is_dir = f_path.is_dir();
            if is_dir {
                walk(f_path.to_str().expect("Could not convert to string"), mt, silence);
            } else {
                rename_file(&f_path, mt, silence);
            }
        }
    } else {
        rename_file(&p, mt, silence)
    }
}

fn main() {
    let mime_types: Value = serde_json::from_str(MIME).expect("Could not parse json");
    // let mut args = env::args();
    // let folder = args.nth(1).expect("Pass a folder!");
    let mut path = String::new();
    let mut silence = Silence::OnlyErrors;
    let mut params = env::args();
    let mut len = params.len();
    while let Some(p) = params.next() {
        print!("{} ", p);
        len -= 1;
        match p.as_str() {
            "-s" => silence = Silence::No,
            "-S" => silence = Silence::Yes,
            "-p" => path = params.next().expect("Use -p with a path!"),
            _ => if len != 0 {
                println!("Enter a folder or file or press enter to pass this folder:");
                std::io::stdin()
                    .read_line(&mut path)
                    .expect("Could not read line");
                path = path.trim().to_string();
                if path == "" {
                    path = p
                }
            },
        }
    }
    walk(&path, &mime_types, &silence);
    // wait for user to press enter
    println!("Press any key to exit");
    std::io::stdin().read_line(&mut path).expect("Could not read line");
}
