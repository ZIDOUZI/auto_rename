use glob::glob;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use tree_magic;
static MIME: &str = std::include_str!("./mimes.json");

fn rename_file(file: &PathBuf, mt: &Value) {
    let mut cp = PathBuf::from(file);
    let f = tree_magic::from_filepath(file);
    let ext = &mt[f];
    let ext = if ext.is_null() {
        ".bin"
    } else {
        ext.as_str().unwrap()
    };
    cp.set_extension(ext);
    if file.eq(&cp) {
        println!("Skipping {:?}", file);
    } else {
        println!("{:?} => {:?}", file, cp);
        fs::rename(file, cp).unwrap_or_else(|_| println!("could not rename {:#?}", file));
    }
}
fn walk(folder: &str, mt: &Value) {
    let p = PathBuf::from(folder);
    if !p.exists() {
        println!("{} does not exist!", p.display())
    }

    if p.is_dir() {
        let pb: PathBuf = p.join("*");
        let star = glob(pb.to_str().unwrap()).expect("Failed to create a glob pattern");
        for i in star {
            let f_path = i.unwrap();
            let is_dir = f_path.is_dir();
            if is_dir {
                walk(f_path.to_str().expect("Could not convert to string"), mt);
            } else {
                rename_file(&f_path, mt);
            }
        }
    } else {
        rename_file(&p, mt)
    }
}
fn main() {
    let mime_types: Value = serde_json::from_str(MIME).expect("Could not parse json");
    let mut args = env::args();
    let folder = args.nth(1).expect("Pass a folder!");
    walk(&folder, &mime_types);
}
