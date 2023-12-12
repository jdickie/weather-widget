/**
 * Module for handling the NOAA Web API
 * https://www.weather.gov/documentation/services-web-api
 */
pub mod weather_api {
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
    use serde_json::Value;
    use std::env;

    #[derive(Default)]
    pub struct GridPointData {
        pub name: String,
        pub start_time: String,
        pub temperature: String,
        pub temperature_unit: String,
        pub short_forecast: String,
    }

    fn get_headers() -> HeaderMap {
        let mut user_agent: String = String::from("weather-widget");
        let user_agent_affix: String = env::var("USER_AGENT").unwrap_or_default();
        user_agent.push_str(&user_agent_affix.as_str());

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).unwrap());
        headers.insert(ACCEPT, HeaderValue::from_static("application/ld+json"));
        headers
    }

    pub async fn get_grid_point(x: &f64, y: &f64) -> Result<GridPointData, reqwest::Error> {
        let client: reqwest::Client = reqwest::Client::new();
        let point_url: String = format!("https://api.weather.gov/points/{},{}", x, y);
        let point_data: Value = client
            .get(point_url)
            .headers(get_headers())
            .send()
            .await?
            .json()
            .await?;
        let gridpoint_url: String = point_data["forecast"].to_string().replace("\"", "");
        let value: Value = client
            .get(gridpoint_url)
            .headers(get_headers())
            .send()
            .await?
            .json()
            .await?;
        let mut output: GridPointData = GridPointData::default();
        output.name = value["periods"][0]["name"].to_string().replace("\"", "");
        output.start_time = value["periods"][0]["startTime"]
            .to_string()
            .replace("\"", "");
        output.temperature = value["periods"][0]["temperature"].to_string();
        output.temperature_unit = value["periods"][0]["temperatureUnit"]
            .to_string()
            .replace("\"", "");
        output.short_forecast = value["periods"][0]["shortForecast"]
            .to_string()
            .replace("\"", "");
        Ok(output)
    }

    #[cfg(test)]
    #[test]
    fn test_get_headers() {
        let header: HeaderMap = get_headers();
        assert_eq!(header.get(USER_AGENT).unwrap(), "weather-widget");
    }
}
