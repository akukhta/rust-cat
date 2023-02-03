use std::os::windows::prelude::FileExt;
use clap::{App, Arg};
use std::fs::File;

struct FileReader {
    path : String,
}

impl FileReader {
    fn new(path : &String) -> FileReader {
        FileReader { 
            path: path.clone(),
        }
    }

    fn print_file(self) -> () {
        let file = File::open(self.path).unwrap();
        let mut pos : u64 = 0;
        let mut byte : [u8; 1] = [0x00];
        
        loop {
            match file.seek_read(&mut byte, pos)
            {
                Ok(readed) => {
                    if readed == 1 {
                        print!("{}", byte[0] as char);
                        pos += 1;
                    }
                    else {
                        break;
                    }
                },
                
                _ => {
                    break;
                }
            } 
        }
    
    }
}


fn main() {
    let matches = App::new("rust-cat").version("0.1").about("Prints file to terminal")
    .arg(Arg::with_name("file").required(true).takes_value(true)).get_matches();
    let file = String::from(matches.value_of("file").unwrap());

    let f_reader = FileReader::new(&file);
    f_reader.print_file();
}
