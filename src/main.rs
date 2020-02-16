use qadata_rs::qafetch::QAMongoClient;
use qadata_rs::qautil::stock_min;

fn main() {
    let mut dataclient = QAMongoClient::new();
    let stock_day = dataclient.get_stock_day("000001", "2020-01-01", "2020-02-01");
    println!("{:#?}", stock_day);
    let stock_min = dataclient.get_stock_min("000001", "2020-01-01", "2020-02-01", "5min");
    println!("{:#?}", stock_min);
    let future_min = dataclient.get_future_min("RBL8", "2020-01-01", "2020-02-01", "5min");
    println!("{:#?}", future_min);
}
