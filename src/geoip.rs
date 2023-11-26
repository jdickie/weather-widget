pub mod geoip {
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::env;

    fn get_geoip_headers() -> HeaderMap {
        let user_agent: String = env::var("USER_AGENT").unwrap();

        let mut map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(3);
        let content: HeaderValue = HeaderValue::from_str("application/json").unwrap();
        map.insert(CONTENT_TYPE, content.clone());
        map.insert(ACCEPT, content.clone());
        map.insert(
            USER_AGENT,
            HeaderValue::from_str(&user_agent.as_str()).unwrap(),
        );
        map
    }

    pub async fn get_geoip_latlon(
        ip_address: &String,
    ) -> Result<HashMap<&str, f64>, reqwest::Error> {
        let user_account: String = env::var("GEOIP_USER_ACCOUNT").unwrap();
        let license_key: String = env::var("GEOIP_LICENSE_KEY").unwrap();
        let geoip_base_uri: String = env::var("GEOIP_BASE_URI").unwrap();
        
        let client = reqwest::Client::builder()
            .default_headers(get_geoip_headers())
            .build()
            .unwrap();
        let mut get_url: String = format!("{}", geoip_base_uri);
        if ip_address == "127.0.0.1" {
          get_url.push_str("/city/me");
        } else {
            let affix = format!("/city/{}", &ip_address.as_str());
            get_url.push_str(&affix);
        }
        let response: Value = client
            .get(get_url)
            .basic_auth(&user_account, Some(&license_key))
            .send()
            .await?
            .json()
            .await?;
        let lat: f64 = response["location"]["latitude"].as_f64().unwrap();
        let lon: f64 = response["location"]["longitude"].as_f64().unwrap();
        Ok(HashMap::from([("lat", lat), ("lon", lon)]))
    }
}
