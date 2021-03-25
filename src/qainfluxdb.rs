use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};

#[async_std::main]
// or #[tokio::main] if you prefer
async fn main() {
    // Connect to db `test` on `http://localhost:8086`
    let client = Client::new("http://192.168.2.116:8086", "test");

    #[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        humidity: i32,
        #[influxdb(tag)] wind_direction: String,
    }

    // Let's write some data into a measurement called `weather`
    let weather_reading = WeatherReading {
        time: Timestamp::Hours(1).into(),
        humidity: 30,
        wind_direction: String::from("north"),
    };

    let write_result = client
        .query(&weather_reading.into_query("weather"))
        .await;
    assert!(write_result.is_ok(), "Write result was not okay");

    // Let's see if the data we wrote is there
    let read_query = Query::raw_read_query("SELECT * FROM weather");

    let read_result = client.query(&read_query).await;
    assert!(read_result.is_ok(), "Read result was not ok");
    println!("{}", read_result.unwrap());
}