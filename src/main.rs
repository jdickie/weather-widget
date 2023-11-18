use reqwest::{header::{USER_AGENT, HeaderMap, ACCEPT, HeaderValue}, Error};
use serde_json::Value;
use std::env;
use std::collections::HashMap;

#[derive(Default)]
struct TempData {
    display_temperature: String,

}

#[derive(Default)]
struct GridPointData {
    name: String,
    start_time: String,
    temperature: String,
    temperature_unit: String,
    short_forecast: String
}

fn convert_to_image(str: String) -> String {
    let weather_image_map: HashMap<&str, &str> = HashMap::from([
        ("Cloudy", "cloudy.img"),
        ("Clear", "clear.img"),
        ("Fog", "patch.img")
    ]);
    let mut img: String = String::new();
    for (key, url) in weather_image_map {
        if str.to_lowercase().contains(&key.to_lowercase()) {
            img = url.to_string();
        }
    }
    img
}

fn format_for_display(grid_point: &GridPointData) -> TempData {
    let img: String = convert_to_image(grid_point.short_forecast.clone());
    let data: TempData = TempData { 
        display_temperature: format!("<div class=\"weather\"><h1>{}</h1> <p><img src=\"{}\"/> <p><strong>{}</strong>{}<br>{}</p></div>", 
        grid_point.name,
        img,
        grid_point.temperature, 
        grid_point.temperature_unit, 
        grid_point.short_forecast) };
    data
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("weather-widget jgrantd@gmail.com"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/ld+json"));
    headers
}



fn get_grid_point(x: &f32, y: &f32) -> Result<GridPointData, Error> {
    let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    let point_url: String = format!("https://api.weather.gov/points/{},{}", x, y);
    let point_result: String = client.get(point_url).headers(get_headers()).send().unwrap().text().unwrap();
    let point_data: Value = serde_json::from_str(&point_result).unwrap();
    let gridpoint_url: String = point_data["forecast"].to_string().replace("\"", "");
    let text: String = client.get(gridpoint_url).headers(get_headers()).send().unwrap().text().unwrap();
    let value: Value = serde_json::from_str(&text).unwrap();
    let mut output: GridPointData = GridPointData::default();
    output.name = value["periods"][0]["name"].to_string().replace("\"", "");
    output.start_time = value["periods"][0]["startTime"].to_string().replace("\"", "");
    output.temperature = value["periods"][0]["temperature"].to_string();
    output.temperature_unit = value["periods"][0]["temperatureUnit"].to_string().replace("\"", "");
    output.short_forecast = value["periods"][0]["shortForecast"].to_string().replace("\"", "");
    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let x: &f32 = &args[1].parse().unwrap();
    let y: &f32 = &args[2].parse().unwrap();
    println!("Looking up {},{}", x, y);
    let s: GridPointData = get_grid_point(x, y).expect("Should have returned grid data");
    let d: TempData  = format_for_display(&s);
    println!("{}", d.display_temperature);
}
