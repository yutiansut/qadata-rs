extern crate parquet;
use std::fs::File;
use std::path::Path;
use parquet::file::reader::{FileReader, SerializedFileReader};


pub fn get_stock_day(path:&str){

    let file = File::open(&Path::new("/path/to/file")).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        println!("{}", record);
    }
}
