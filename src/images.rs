pub mod conversion {
    use std::collections::HashMap;
    
    pub fn convert_to_image(str: &String) -> String {
        let weather_image_map: HashMap<&str, &str> = HashMap::from([
            ("Cloudy", "cloudy.img"),
            ("Clear", "clear.img"),
            ("Fog", "patch.img"),
            ("Sunny", "sun.img"),
            ("Rain", "rain.png")
        ]);
        let mut img: String = String::new();
        for (key, url) in weather_image_map {
            if str.to_lowercase().contains(&key.to_lowercase()) {
                img = url.to_string();
            }
        }
        if img.len() < 1 {
            img = "default.img".into();
        }
        img
    }
}

#[cfg(test)]
mod image_tests {
    use super::conversion::convert_to_image;

    #[test]
    fn test_match_sunny() {
        let should_be_sunny = convert_to_image(&String::from("It is quite sunny today"));
        assert_eq!(should_be_sunny, "sun.img");
    }

    #[test]
    fn test_match_rain() {
        let should_be_rain = convert_to_image(&String::from("rainy"));
        assert_eq!(should_be_rain, "rain.png");
    }

    #[test]
    fn test_match_fog() {
        let should_be_foggy = convert_to_image(&String::from("foggy"));
        assert_eq!(should_be_foggy, "patch.img");
    }

    #[test]
    fn test_default() {
        let should_be_sunny = convert_to_image(&String::from("foo"));
        assert_eq!(should_be_sunny, "default.img");
    }
}
