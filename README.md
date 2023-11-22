# Weather Widget

Experimental Rust app that will generate an HTML display for the weather. Location based on IP address.

Uses the [NOAA Weather API](https://www.weather.gov/documentation/services-web-api)

## How to build

```
cargo build
```

### Building for AWS Lambda

I'm using the arm64 release target:

```
cargo lambda build --release --arm64
```

## Tests

Tests are written inside the modules they are based on (For now).

```
cargo test
```

## Contributing

Contributions welcome - please note we're using the [Mozilla Public License Version 2.0](LICENSE)