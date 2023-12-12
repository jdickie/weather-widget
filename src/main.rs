use lambda_http::{
    request::RequestContext, run, service_fn, Body, Error, Request, RequestExt, Response,
};
mod geoip;
mod images;
mod noaa;
use std::collections::HashMap;

#[derive(Default)]
struct TempData {
    display_temperature: String,
}

fn produce_css() -> Result<Response<Body>, Error> {
    let css: &str = ".weather { text-align: center; background-color: grey; height: 100%; width: 100%; } \
    .weather-name { width: 75%; height: 50%; margin: auto; } \
.weather-name h1 { font-size: 100; }\
.weather-temp { width: 50%; margin: auto; }\
.weather-temp h2 { font-size: 85; }";
    let resp: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "text/css")
        .body(css.into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn format_for_display(grid_point: &noaa::weather_api::GridPointData) -> TempData {
    let img: String = images::conversion::convert_to_image(&grid_point.short_forecast);
    let data: TempData = TempData {
        display_temperature: format!("<head><link type=\"text/css\" rel=\"stylesheet\" href=\"weather.css\" /> \
        </head><title>Weather App</title> \
        <div class=\"weather\" style=\"background-image: url({});\"> \
        <div class=\"weather-name\"><h1>{}</h1></div> \
        <div class=\"weather-temp\"><h2>{}{}</h2><h4>{}</h4></div>\
        </div>",
        img,
        grid_point.name,
        grid_point.temperature,
        grid_point.temperature_unit,
        grid_point.short_forecast) };
    data
}

fn get_ip_address_or_default(event: Request) -> String {
    let request_context: lambda_http::request::RequestContext = event.request_context();
    let source_ip = match request_context {
        RequestContext::ApiGatewayV2(req) => req.http.source_ip.unwrap(),
        RequestContext::Alb(_) => String::from("73.212.162.22"),
        RequestContext::ApiGatewayV1(req) => req.identity.source_ip.unwrap(),
        RequestContext::WebSocket(req) => req.identity.source_ip.unwrap(),
    };
    source_ip
}

// Primary function that is called from the main handler
// HTTP requests should have an x and y query string parameter. Will upgrade to POST bodies in the future.
//
async fn produce_html(event: Request) -> Result<Response<Body>, Error> {
    if event.raw_http_path() == "/" {
        let ip = get_ip_address_or_default(event);
        let lat_long: HashMap<&str, f64> = geoip::geoip::get_geoip_latlon(&ip).await?;
        let grid: noaa::weather_api::GridPointData = noaa::weather_api::get_grid_point(
            lat_long.get("lat").unwrap(),
            lat_long.get("lon").unwrap(),
        )
        .await?;

        let d: TempData = format_for_display(&grid);
        let resp: Response<Body> = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(d.display_temperature.into())
            .map_err(Box::new)?;
        Ok(resp)
    } else if event.raw_http_path() == "/weather.css" {
        produce_css()
    } else {
        let resp: Response<Body> = Response::builder()
            .status(204)
            .body("Event".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(produce_html)).await
}
