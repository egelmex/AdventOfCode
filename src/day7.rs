use core::fmt;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    clone,
    env::current_exe,
    fmt::Write,
    rc::Rc,
    vec,
};

use rand::distributions::uniform::SampleBorrow;

use crate::read_lines;

pub fn part1() {
    let lines = read_lines::read_lines("inputs/mine/day7.txt")
        .unwrap()
        .map(|x| x.unwrap());
    let root = construct(lines);

    let mut to_proccess = vec![root];
    let mut count = 0;
    while !to_proccess.is_empty() {
        let next = to_proccess.pop().unwrap();

        if next.borrow_mut().size() <= 100000 {
            count += next.borrow_mut().size();
        }
        for c in next.clone().borrow_mut().dirs.clone() {
            to_proccess.push(c.clone());
        }
    }
    println!("{}", count);
}

pub fn part2() {}

struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

fn construct(lines: impl Iterator<Item = String>) -> Rc<RefCell<Dir>> {
    let root = Dir::new("", None);
    let mut current = root.clone();

    for line in lines {
        dbg!(&line);
        if line.starts_with("$ cd") {
            let path = line.split(" ").nth(2).unwrap();
            if path == ".." {
                let parent = current.clone().borrow_mut().parent.clone();
                let parent = parent.as_ref().unwrap();
                current = parent.clone();
            } else {
                let tmp = current.clone();
                current = Dir::get_child(current.clone(), path);
                if !tmp.borrow_mut().dirs.contains(&current) {
                    tmp.borrow_mut().dirs.push(current.clone());
                }
            }
        } else if line.starts_with("$") {
            //skip
        } else if line.starts_with("dir") {
            let path = line.split(" ").nth(1).unwrap();
            let new = Dir::get_child(current.clone(), path);
            if !current.borrow_mut().dirs.contains(&new) {
                current.borrow_mut().dirs.push(new.clone());
            }
        } else {
            let mut p = line.split(" ");
            let size = p.next().unwrap().parse::<u32>().unwrap();
            let name = p.next().unwrap();
            current.borrow_mut().files.push(File::New(name, size));
        }
    }

    root
}

impl Dir {
    fn new(name: &str, parent: Option<Rc<RefCell<Dir>>>) -> Rc<RefCell<Self>> {
        let d = Dir {
            name: name.to_string(),
            parent: parent,
            dirs: vec![],
            files: vec![],
        };
        Rc::new(RefCell::new(d))
    }

    fn get_child(parent: Rc<RefCell<Dir>>, path: &str) -> Rc<RefCell<Dir>> {
        let new = Dir::new(path, Some(parent.clone()));
        // set parents child

        for child in &parent.borrow_mut().dirs {
            let name = &child.borrow_mut().name;
            if name == path {
                return child.clone();
            }
        }
        new
    }

    fn size(&self) -> u32 {
        let s: u32 = self.files.clone().into_iter().map(|x| x.size).sum();
        let c: u32 = self.dirs.iter().map(|x| x.borrow_mut().size()).sum();
        //dbg!(c);
        s + c
    }
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dir {} {} dirs:[{:#?}] files:[{:#?}]",
            self.name,
            self.size(),
            self.dirs,
            self.files
        )
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File {} {}]", self.name, self.size)
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

#[derive(Clone)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn New(name: &str, size: u32) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}
