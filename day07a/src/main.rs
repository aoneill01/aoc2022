#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    directories: Vec<Directory<'a>>,
    files: Vec<File<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Directory<'a> {
        Directory {
            name,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    fn find_dir(self: &mut Self, name: &str) -> Option<&'a mut Directory> {
        self.directories.iter_mut().find(|d| d.name.eq(name))
    }
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: i32,
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: i32) -> File {
        File { name, size }
    }
}

fn main() {
    let mut root = Directory::new("root");

    {
        let cwd = &mut root;

        let file = File::new("hey", 5);
        cwd.files.push(file);

        // let dir = Directory::new("one");
        // cwd.directories.push(dir);
        // let cwd = root.find_dir("one").unwrap();

        // let file = File::new("another", 10);
        // cwd.files.push(file);
    }

    println!("{:?}", root);

    // for line in include_str!("../input.txt").lines() {}
}
