use ron::de;
use std::fs::File;
use std::io::BufReader;

pub fn read_template_file<T: for<'de> serde::Deserialize<'de>>(filename: String) -> Vec<T> {
    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open {}:\n  {}", &filename, e);
            panic!();
        }
    };
    let reader = BufReader::new(f);
    let h: Vec<T> = match de::from_reader(reader) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not parse RON-file {}:\n  {}", &filename, e);
            panic!();
        }
    };
    h
}
