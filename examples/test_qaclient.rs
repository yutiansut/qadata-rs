use qadata_rs::qamongo::QAMongoClient;
use qadata_rs::qaparquet::QAParquetClient;
use mifi_rs::kline::stock_min;

// use qadata_rs::qaparquet::RecordWriter;
//
// #[derive(ParquetRecordWriter)]
// struct index<'a> {
//     low: f64,
//     open: f64,
//     close: f64,
//     high: f64,
//     num_trades: f64,
//     volume: f64,
//     total_turnover: f64,
//     date: i64,
//     order_book_id: String
// }
fn main(){
    //let qamc= QAMongoClient::new("mongodb://192.168.2.117", "quantaxis");


    // let mut dataclient = QAMongoClient::new("mongodb://192.168.2.118:27017", "quantaxis");
    // let stock_day = dataclient.get_stock_day("000001", "2020-01-01", "2020-02-01");
    // println!("{:#?}", stock_day);
    // let stock_min:Vec<stock_min> = dataclient.get_stock_min("000001", "2020-01-01", "2020-02-01", "5min");
    // println!("{:#?}", stock_min);
    // //
    // //
    // // let future_day = dataclient.get_stock_day("RBL8", "2020-01-01", "2020-02-01");
    // // println!("{:#?}", future_day);
    //
    //
    // let future_min = dataclient.get_future_min("RBL8", "2020-06-01", "2020-07-01", "1min");
    // println!("{:#?}", future_min);
    //

    let client_pq =  QAParquetClient::new("E:\\QARS\\qadata-rs\\examples\\equity_index.parquet".to_string());
    let res_data = client_pq.get_data();
    for i in res_data{
        println!("{:#?}", i)
    }
}