mod mime;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use tree_magic;
use crate::mime::MIME_MAP;

#[derive(PartialEq)]
enum Silence {
    No,
    OnlyErrors,
    Yes,
}

fn rename_file(file: &PathBuf, mt: &HashMap<&str, &str>, silence: &Silence) {
    let mut cp = PathBuf::from(file);
    let f = tree_magic::from_filepath(file);
    let ext = mt.get(&f.as_str());
    match ext {
        None => {
            if silence != &Silence::Yes {
                println!("Could not find extension for {:?}", file);
            }
        }
        Some(ext) => {
            cp.set_extension(*ext);
            if file.eq(&cp) {
                if silence == &Silence::No {
                    println!("Skipping {:?}", file);
                }
            } else {
                if silence != &Silence::Yes {
                    println!("{:?} => {:?}", file, cp.file_name().unwrap());
                }
                fs::rename(file, cp).unwrap_or_else(|_| if silence != &Silence::Yes { println!("could not rename {:#?}", file) });
            }
        }
    }
}

fn walk(path: PathBuf, mt: &HashMap<&str, &str>, silence: &Silence) {

    if path.is_dir() {
        fs::read_dir(path).unwrap().for_each(|entry| {
            let p = entry.unwrap().path();
            if p.is_dir() {
                walk(p, mt, silence);
            } else {
                rename_file(&p, mt, silence);
            }
        });
    } else {
        rename_file(&path, mt, silence)
    }
}

fn main() {
    let mime_types: HashMap<&str, &str> = HashMap::from(MIME_MAP);
    // let mut args = env::args();
    // let folder = args.nth(1).expect("Pass a folder!");
    let mut path = String::new();
    let mut silence = Silence::OnlyErrors;
    let mut params = env::args();
    let mut len = params.len();
    while let Some(p) = params.next() {
        len -= 1;
        match p.as_str() {
            "-s" => silence = Silence::No,
            "-S" => silence = Silence::Yes,
            "-p" => path = params.next().expect("Use -p with a path!"),
            _ => if len == 0 {
                path = p;
                println!("Using path: {}", path);
                println!("Press enter to continue or type a new path:");
                let mut tmp = String::new();
                read_line(&mut tmp);
                if tmp != "" {
                    path = tmp;
                }
            },
        }
    }

    loop {
        if path.starts_with("\"") && path.ends_with("\"") {
            path = path[1..path.len() - 1].to_string();
            println!("Using path: {}", path)
        }
        let p = PathBuf::from(&path);
        if !p.exists() && silence != Silence::Yes {
            println!("{} does not exist!", p.display())
        }
        walk(p, &mime_types, &silence);

        println!("Enter a folder or file or press enter to exit:");
        read_line(&mut path);
        if path == "" {
            break;
        }
    }
}

fn read_line(path: &mut String) {
    path.clear();
    std::io::stdin()
        .read_line(path)
        .expect("Could not read line");
    // remove trailing \r\n or \n
    path.pop();
    if path.ends_with("\r") {
        path.pop();
    }
}

/*fn main() {
    let mut s = String::new();
    for i in env::args() {
        println!("{}", i);
    }
    std::io::stdin().read_line(&mut s).expect("Could not read line");
}
*/