use std::path::Prefix::Verbatim;


pub use mongodb::{
    bson::{doc, Bson, Document},
    error::Result,
    options::{
        DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateModifications,
        UpdateOptions,ClientOptions
    },
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection, Cursor, Database},
};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};


use mifi_rs::kline::{future_day, future_min, stock_day, stock_min};

fn to_timestamp(date: String) -> i64{
    Utc.datetime_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S")
    .unwrap()
    .timestamp() -28800
}


pub struct QAMongoClient {
    pub uri: String,
    pub database: Database,
}

impl QAMongoClient {
    pub fn new(uri: &str, database: &str) -> Self {
        // let mut client_options = ClientOptions::(uri).unwrap();
        // client_options.app_name = Some("QUANTAXIS".to_string());
        // let client = Client::with_options(client_options).unwrap();

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(database);
        Self {
            uri: uri.to_string(),
            database: db,
        }
    }
    pub fn get_stocks_day(&mut self, code: Vec<String>, start: &str, end: &str) -> Vec<stock_day> {
        let collection = self.database.collection("stock_day");
        //println!("start {} end {}", start, end);
        let filter = doc! {"code": {"$in": code},
                                            "date": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().sort(doc!{"date_stamp":1}).batch_size(Option::from(10000000)).build();
        let cursor = collection.find(filter, find_options).unwrap();
        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: stock_day = bson::from_document(document).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }

    pub fn get_stock_day(&mut self, code: &str, start: &str, end: &str) -> Vec<stock_day> {
        let collection = self.database.collection("stock_day");

        let filter = doc! {"code": code,
                                            "date_stamp": {"$gte": to_timestamp(start.to_string()), "$lte":  to_timestamp(end.to_string())}};
        let find_options = FindOptions::builder().sort(doc!{"date_stamp":1}).build();
        let cursor = collection.find(filter, find_options).unwrap();
        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: stock_day = bson::from_document(document).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }

    pub fn get_stock_min(&mut self, code: &str, start: &str, end: &str, frequence: &str) -> Vec<stock_min> {
        let collection = self.database.collection("stock_min");

        let filter = doc! {"code":code, "type": frequence, "datetime": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().sort(doc!{"datetime":1}).build();
        let cursor = collection.find(filter, find_options).unwrap();

        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: stock_min = bson::from_document(document).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }
    pub fn get_future_day(&mut self, code: &str, start: &str, end: &str) -> Vec<future_day> {
        let collection = self.database.collection("future_day");

        let filter = doc! {"code": code,
                                            "date": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().sort(doc!{"date":1}).build();
        let cursor = collection.find(filter, find_options).unwrap();
        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: future_day = bson::from_document(document).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }
    pub fn get_future_min(&mut self, code: &str, start: &str, end: &str, frequence: &str) -> Vec<future_min> {
        let collection = self.database.collection("future_min");

        let filter = doc! {"code":code, "type": frequence, "datetime": {"$gte": start, "$lte": end}};
        let find_options = FindOptions::builder().sort(doc!{"datetime":1}).build();
        let cursor = collection.find(filter, find_options).unwrap();

        let mut res = Vec::new();
        for result in cursor {
            match result {
                Ok(document) => {
                    let u: future_min = bson::from_document(document).unwrap();
                    res.push(u);
                }
                Err(e) => { println!("ERROR"); } //return Err(e.into()),
            }
        }
        res
    }

    pub fn get_collection(&mut self, collection: &str) -> Collection{
        self.database.collection(collection)
    }

    // pub fn get_stock_realtime(&mut self, code: Vec<&str>, start: &str, end: &str, frequence: &str) -> Vec<future_min> {
    //     let collection = self.database.collection("stock_realtime_min");
    //
    //     let filter = doc! {"code": {"$in": Array(code)}, "type": frequence, "datetime": {"$gte": start, "$lte": end}};
    //     let find_options = FindOptions::builder().build();
    //     let cursor = collection.find(filter, find_options).unwrap();
    //
    //     let mut res = Vec::new();
    //     for result in cursor {
    //         match result {
    //             Ok(document) => {
    //                 let u: future_min = bson::from_bson(bson::Bson::Document(document)).unwrap();
    //                 res.push(u);
    //             }
    //             Err(e) => { println!("ERROR"); } //return Err(e.into()),
    //         }
    //     }
    //     res
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_totimestamp(){
        println!("{:#?}",to_timestamp("2020-01-01".to_string()));
        //assert_eq!(1577836800)
    }
}