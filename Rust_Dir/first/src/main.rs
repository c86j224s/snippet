use std::vec::Vec;
use std::fs;
use std::io;
use std::path::Path;
use std::ffi::OsStr;

enum DirEntType {
    File,
    Directory,
}

struct DirEnt {
    name: String,
    ent_size: u64,
    ent_type: DirEntType,
    ent_childs: Vec<DirEnt>,
}

impl DirEnt {
    fn new_from_path(ent_path: &Path) -> io::Result<DirEnt> {
        let ent_meta = match fs::metadata(ent_path) {
            Ok(v) => v,
            Err(e) => {
                println!("Metadata Fail : {:?}", ent_path);
                return Err(e);
            }
        };

        let mut ent_size = ent_meta.len();
        let mut ent_childs = Vec::new();

        if ent_meta.is_dir() {
            for child in try!(fs::read_dir(ent_path)) {
                let child_ent = try!(child);
                let child_dirent = try!(DirEnt::new_from_path(&child_ent.path()));

                ent_size += child_dirent.ent_size;
                ent_childs.push(child_dirent);
            }
        }

        Ok(DirEnt {
            name: String::from(
                ent_path
                    .file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_str()
                    .unwrap_or(""),
            ),
            ent_size: ent_size,
            ent_type: if ent_meta.is_dir() {
                DirEntType::Directory
            } else {
                DirEntType::File
            },
            ent_childs: ent_childs,
        })
    }

    fn print(&self) {
        self.print_with_depth(0);
    }

    fn print_with_depth(&self, depth: u32) {
        let mut ds = String::new();
        for _ in 0..(depth * 2) {
            ds.push_str(" ");
        }

        println!(
            "{}\t{}\t{}{:?}",
            match self.ent_type {
                DirEntType::Directory => "D",
                DirEntType::File => "F",
            },
            self.ent_size,
            ds,
            self.name
        );

        for child in &self.ent_childs {
            child.print_with_depth(depth + 1);
        }
    }
}

fn main() {
    match DirEnt::new_from_path(&Path::new("/Users/allthatcode")) {
        Ok(ent) => ent.print(),
        Err(err) => println!("FAIL : {:?}", err),
    }
}
