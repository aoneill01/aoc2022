use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct Directory {
    name: String,
    directories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            directories: Vec::new(),
            files: Vec::new(),
            parent: None,
        }
    }

    // fn find_dir(self: &mut Self, name: &str) -> Option<&'a mut Directory> {
    //     self.directories.iter_mut().find(|d| d.name.eq(name))
    // }
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: &str, size: i32) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Directory::new("/")));

    {
        let n = Directory::new("foo");
        root.borrow_mut().directories.push(Rc::new(RefCell::new(n)));
        let tmp = root.borrow();
        let mut cwd = tmp.directories.first().unwrap().borrow_mut();
        let n = Directory::new("another");
        cwd.directories.push(Rc::new(RefCell::new(n)));
    }

    println!("{:?}", root);
}
