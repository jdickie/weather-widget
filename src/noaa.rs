/**
 * Module for handling the NOAA Web API
 * https://www.weather.gov/documentation/services-web-api
 */
pub mod weather_api {
  use serde_json::Value;
  use reqwest::header::{USER_AGENT, HeaderMap, ACCEPT, HeaderValue};


#[derive(Default)]
pub struct GridPointData {
    pub name: String,
    pub start_time: String,
    pub temperature: String,
    pub temperature_unit: String,
    pub short_forecast: String
}

  fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("weather-widget jgrantd@gmail.com"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/ld+json"));
    headers
}

pub async fn get_grid_point(x: &f32, y: &f32) -> Result<GridPointData, reqwest::Error> {
    let client: reqwest::Client = reqwest::Client::new();
    let point_url: String = format!("https://api.weather.gov/points/{},{}", x, y);
    let point_result: String = client.get(point_url).headers(get_headers()).send().await?.text().await?;
    println!("{}", point_result);
    let point_data: Value = serde_json::from_str(&point_result).unwrap();
    let gridpoint_url: String = point_data["forecast"].to_string().replace("\"", "");
    println!("{}", gridpoint_url);
    let text: String = client.get(gridpoint_url).headers(get_headers()).send().await?.text().await?;
    let value: Value = serde_json::from_str(&text).unwrap();
    let mut output: GridPointData = GridPointData::default();
    output.name = value["periods"][0]["name"].to_string().replace("\"", "");
    output.start_time = value["periods"][0]["startTime"].to_string().replace("\"", "");
    output.temperature = value["periods"][0]["temperature"].to_string();
    output.temperature_unit = value["periods"][0]["temperatureUnit"].to_string().replace("\"", "");
    output.short_forecast = value["periods"][0]["shortForecast"].to_string().replace("\"", "");
    Ok(output)
}
}