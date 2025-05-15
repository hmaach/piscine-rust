use std::path::Path;
use std::fs::File;
use std::fs::read_to_string;
use std::io::ErrorKind;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {

    File::open(path).unwrap_or_else(|err|{
        if err.kind() == ErrorKind::NotFound{
            File::create(path).unwrap_or_else(|err|{
                panic!("Error creating the file !");
            })
        }else{
            panic!("Unexpected error while trying to read the file!");
        }
    });
}
