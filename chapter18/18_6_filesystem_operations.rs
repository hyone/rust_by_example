use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    f.read_to_string(&mut s).map(|_| s)
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = try!(File::create(path));
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    OpenOptions::new().write(true).create(true).open(path).map(|_| ())
}

fn main() {
    println!("`mkdir a`");
    match fs::create_dir("a") {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(_)  => {},
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });

    println!("`mkdir -p a/c/d`");
    fs::create_dir_all("a/c/d").unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|e| {
            println!("! {:?}", e.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(s)  => println!("> {}", s),
    }

    println!("`ls a`");
    match fs::read_dir("a") {
        Err(e)    => println!("! {:?}", e.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });

    println!("`rmdir a/c/d`");
    fs::remove_dir("a/c/d").unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });

    println!("`rm -rf a/`");
    fs::remove_dir_all("a").unwrap_or_else(|e| {
        println!("! {:?}", e.kind());
    });
}
