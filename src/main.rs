use serde::{Deserialize, Serialize};
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use std::io::{self, Read};

#[derive(Debug, Deserialize, Serialize)]
struct Current {
    time: String,
    interval: i32,
    temperature_2m: f32,
    relative_humidity_2m: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct CurrentUnits {
    temperature_2m: String,
    relative_humidity_2m: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MeteoResponse {
    latitude: f64,
    longitude: f64,
    current: Current,
    current_units: CurrentUnits,
}

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    city: String,
    country: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GeocodeResponse {
    address: Address,
}

#[tokio::main]
async fn main() {
    println!("Input your country below:");
    let mut user_country = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_country);
    println!("Your country: {}", user_country);
    println!("Input your city below:");
    let mut user_city = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_city);

    println!("Your city: {}", user_city);

    let openmeteourl = format!(
        "https://api.open-meteo.com/v1/forecast?latitude=62.7945&longitude=22.8282&current=temperature_2m,relative_humidity_2m&forecast_days=1"
    );
    let geocodeurl = format!(
        "https://geocode.maps.co/reverse?lat=62.798153&lon=22.820572&api_key=65c42bcc48ad9382214205esvdbd9b9"
    );

    impl MeteoResponse {
        fn meteo_print_all(&self) {
            //println!("Current statistic for Lat: {} and Lon: {}", self.latitude, self.longitude);
            println!("Time: {}", self.current.time);
            println!("Temperature: {}{}", self.current.temperature_2m, self.current_units.temperature_2m);
            println!("Humidity: {}{}", self.current.relative_humidity_2m, self.current_units.relative_humidity_2m);
            pause();
        }
    }

    impl GeocodeResponse {
        fn geocode_print_all(&self) {
            println!("Current statistics for {}, {}\n", self.address.city, self.address.country);
        }
    }

    let client = reqwest::Client::new();
    let meteoresponse = client
        .get(openmeteourl)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    let geocoderesponse = client
        .get(geocodeurl)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    
    // Response from geocode.maps.co
    match geocoderesponse.status() {
        reqwest::StatusCode::OK => {
            match geocoderesponse.json::<GeocodeResponse>().await {
                Ok(parsed) => parsed.geocode_print_all(),
                Err(_) => println!("Nope")
            };
        }
        _other => {
            panic!("Panic")
        }
    };

    // Response from api.open-meteo.com
    match meteoresponse.status() {
        reqwest::StatusCode::OK => {
            match meteoresponse.json::<MeteoResponse>().await {
                Ok(parsed) => parsed.meteo_print_all(),
                Err(_) => println!("Nope")
            };
        }
        _other => {
            panic!("Panic")
        }
    };

    fn pause() {
        io::stdin().read(&mut [0]).unwrap();
    }
}