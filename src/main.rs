use mongodb::{Client, options::ClientOptions};
use mongodb::options::FindOptions;
use bson::{Bson, oid};

use bson::Document;
use bson::{doc, bson};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct stock_day {

    open : f64,
    close : f64,
    high : f64,
    low : f64,
    vol : f64,
    amount : f64,
    date : String,
    code : String,
//    date_stamp : f64
}

#[derive(Serialize, Deserialize, Debug)]
struct stock_min {
    open : f64,
    close : f64,
    high : f64,
    low : f64,
    vol : f64,
    amount : f64,
    date : String,
    datetime: String,
    code : String,
//    date_stamp : f64,
//    time_stamp : f64,
    #[serde(rename = "type")]
    frequence: String,
}


fn main() {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();


    client_options.app_name = Some("QUANTAXIS".to_string());


    let client = Client::with_options(client_options).unwrap();

    let db = client.database("quantaxis");
    let collection = db.collection("stock_min");

    let filter = doc! {"code": "000001", "type": "5min", "datetime": {"$gte": "2020-01-02 09:00:00"}};
    let find_options = FindOptions::builder().build();
    let cursor = collection.find(filter, find_options).unwrap();

    for result in cursor {
        match result {
            Ok(document) => {
//                let x  = document.get("code").and_then(Bson::as_str);
                let u:stock_min = bson::from_bson(bson::Bson::Document(document)).unwrap();
                println!("code: {:#?}", u);

            }
            Err(e) => {println!("ERROR");} //return Err(e.into()),
        }
    }
}
