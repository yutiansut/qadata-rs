//! Provide storage statistics for parquet files
use arrow_deps::parquet::{
    self,
    file::reader::{FileReader, SerializedFileReader},
    schema,
    record::Row,
};
pub use arrow_deps::parquet_derive;
pub use arrow_deps::parquet::record::RecordWriter;
use std::fs::File;
use std::path::Path;


pub struct QAParquetClient{
    filepath : String,
}



impl QAParquetClient{
    pub fn new(filepath: String) -> QAParquetClient{
        QAParquetClient{filepath}
    }
    pub fn get_data(&self) -> Vec<Row> {
        let mut data: Vec<Row> = vec![];
        let file = File::open(&Path::new(&self.filepath)).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let mut iter = reader.get_row_iter(None).unwrap();
        while let Some(record) = iter.next() {
            //println!("{}", record);
            data.push(record);
        }
        data
    }
}

