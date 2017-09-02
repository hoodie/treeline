extern crate treeline;

use treeline::*;

use std::{env, io, fs, fmt};
use std::path::Path;

fn label<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
}

fn tree<P: AsRef<Path>>(p: P) -> io::Result<Tree<String>> {
    let result = fs::read_dir(&p)?
        .into_iter()
        .filter_map(|e| e.ok())
        .fold(Tree::root(label(p.as_ref().canonicalize()?)),
              |mut root, entry| {
            let dir = entry.metadata().unwrap();
            if dir.is_dir() {
                root.push(tree(entry.path()).unwrap());
            } else {
                root.push(Tree::root(label(entry.path())));
            }
            root
        });
    Ok(result)
}

pub struct SimpleTreeConfig;
impl TreeConfig for SimpleTreeConfig {
    const LINE: &'static str = "-";
    const LAST: &'static str = "+";
    const JOIN: &'static str = "+";
    const BAR: &'static str = "|";
}

pub struct SimpleTree<D: fmt::Display>(pub Tree<D>);

impl<D: fmt::Display> fmt::Display for SimpleTree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write(f, &SimpleTreeConfig)
    }
}

pub struct TightTreeConfig;
impl TreeConfig for TightTreeConfig {
    const LINE: &'static str = "─";
    const BAR: &'static str = "│";
    const DEPTH: usize = 1;
}

pub struct TightTree<D: fmt::Display>(pub Tree<D>);

impl<D: fmt::Display> fmt::Display for TightTree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write(f, &TightTreeConfig)
    }
}

pub struct EmojiTreeConfig;
impl TreeConfig for EmojiTreeConfig {
    const LINE: &'static str ="⟼";
    const LAST: &'static str ="💔";
    const JOIN: &'static str ="💖";
    const BAR:  &'static str ="🇮🇪|";
}

pub struct EmojiTree<D: fmt::Display>(pub Tree<D>);

impl<D: fmt::Display> fmt::Display for EmojiTree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write(f, &EmojiTreeConfig)
    }
}


fn main() {
    let dir = env::args().nth(1).unwrap_or(String::from("."));
    match tree(dir) {
        Ok(tree) => println!("{}", tree),
        //Ok(tree) => println!("{}", SimpleTree(tree)),
        Err(err) => println!("error: {}", err)
    }

    let dir = env::args().nth(1).unwrap_or(String::from("."));
    match tree(dir) {
        Ok(tree) => println!("{}", TightTree(tree)),
        Err(err) => println!("error: {}", err)
    }
}
