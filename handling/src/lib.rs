use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let fs = OpenOptions::new().create(true).append(true).open(path);
    match fs {
        Ok(mut file) => match File::write(&mut file, content.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        },
        Err(err) => panic!("{err}"),
    };
}
