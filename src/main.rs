use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
mod images;
mod noaa;

#[derive(Default)]
struct TempData {
    display_temperature: String,
}

fn format_for_display(grid_point: &noaa::weather_api::GridPointData) -> TempData {
    let img: String = images::conversion::convert_to_image(&grid_point.short_forecast);
    let data: TempData = TempData {
        display_temperature: format!("<div class=\"weather\"><h1>{}</h1> <p><img src=\"{}\"/> <p><strong>{}</strong>{}<br>{}</p></div>", 
        grid_point.name,
        img,
        grid_point.temperature,
        grid_point.temperature_unit,
        grid_point.short_forecast) };
    data
}

// Primary function that is called from the main handler
// HTTP requests should have an x and y query string parameter. Will upgrade to POST bodies in the future.
//
async fn produce_html(event: Request) -> Result<Response<Body>, Error> {
    let x: f32 = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("x"))
        .unwrap_or("39.1")
        .parse()
        .unwrap();
    let y: f32 = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("y"))
        .unwrap_or("-76")
        .parse()
        .unwrap();
    let grid: noaa::weather_api::GridPointData = noaa::weather_api::get_grid_point(&x, &y).await?;

    let d: TempData = format_for_display(&grid);
    let resp: Response<Body> = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(d.display_temperature.into())
        .map_err(Box::new)?;
    Ok(resp)
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

#[cfg(test)]
mod main_test {
    use super::*;

    #[tokio::test]
    async fn test_produce_html_defaults() {
        let request = lambda_http::Request::new("".into());

        let _response = produce_html(request).await.expect("Expected no errors");
    }
}
