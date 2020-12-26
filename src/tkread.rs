// extern crate clickhouse_rs;
// extern crate futures;
//
// use clickhouse_rs::{Pool, types::Block};
// use futures::Future;
//
// pub fn main() {
//     let database_url = "tcp://default@127.0.0.1:9000?compression=lz4&ping_timeout=42ms";
//     let ddl = "
//         CREATE TABLE IF NOT EXISTS payment (
//             customer_id  UInt32,
//             amount       UInt32,
//             account_name Nullable(FixedString(3))
//         ) Engine=Memory";
//     let l1slice = "
//         CREATE TABLE IF NOT EXISTS TRANSACTION (
//             BuyPrices: Array(Float64),
//             BuyVols: Vec<UInt32>,
//             SellPrices: Vec<Float64>,
//             SellVols: Vec<UInt32>,
//             code: String,
//             open: Float64,
//             high: Float64,
//             low: Float64,
//             close: Float64,
//             amount: UInt64,
//             productid: UInt32,
//             tickcount: UInt32,
//             time: DateTime,
//             vol: UInt64,
//
//         ) Engine=Memory";
//
//     let block = Block::new()
//         .column("customer_id", vec![1_u32, 3, 5, 7, 9])
//         .column("amount", vec![2_u32, 4, 6, 8, 10])
//         .column("account_name", vec![Some("foo"), None, None, None, Some("bar")]);
//
//     let pool = Pool::new(database_url);
//
//     let done = pool
//         .get_handle()
//         .and_then(move |c| c.execute(ddl))
//         //.and_then(move |c| c.insert("payment", block))
//         .and_then(move |c| c.query("SELECT * FROM payment").fetch_all())
//         .and_then(move |(_, block)| {
//             for row in block.rows() {
//                 let id: u32 = row.get("customer_id")?;
//                 let amount: u32 = row.get("amount")?;
//                 let name: Option<&str> = row.get("account_name")?;
//                 println!("Found payment {}: {} {:?}", id, amount, name);
//             }
//             Ok(())
//         })
//         .map_err(|err| eprintln!("database error: {}", err));
//     tokio::run(done)
// }