use qadata_rs::qafetch::QAMongoClient;
use mifi_rs::kline::stock_min;

fn main() {
    let mut dataclient = QAMongoClient::new("mongodb://192.168.2.118:27017", "quantaxis");
    let stock_day = dataclient.get_stock_day("000001", "2020-01-01", "2020-02-01");
    println!("{:#?}", stock_day);
    let stock_min:Vec<stock_min> = dataclient.get_stock_min("000001", "2020-01-01", "2020-02-01", "5min");
    println!("{:#?}", stock_min);
    //
    //
    // let future_day = dataclient.get_stock_day("RBL8", "2020-01-01", "2020-02-01");
    // println!("{:#?}", future_day);


    let future_min = dataclient.get_future_min("RBL8", "2020-06-01", "2020-07-01", "1min");
    println!("{:#?}", future_min);
}
