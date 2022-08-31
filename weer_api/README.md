# Weer api
A wrapper library for using [Weather API](https://www.weatherapi.com/)

*This is an unofficial library*

## Usage
Put this in your Cargo.toml:
```toml
[dependencies]
weer_api = "0.1.0"
```

### Examples

#### Get forecast
> ```rs
> use weer_api::{*, chrono::{Utc, TimeZone}};
> 
> let client = Client::new("api_key", true);
> let result = client.forecast()
>     .query(Query::City("London".to_string()))
>     .dt(Utc.ymd(2022, 08, 21).and_hms(0, 0, 0))
>     .lang(Language::Spanish)
>     .call();
> 
> assert!(result.is_ok())
> ```

#### Get future 
> ```rs
> use weer_api::{*, chrono::{Utc, TimeZone}};
> 
> let client = Client::new("api_key", true);
> let result = client.future()
>     .query(Query::Coords(48.8567, 2.3508))
>     .dt(Utc.ymd(2022, 09, 21).and_hms(0, 0, 0))
>     .lang(Language::Spanish)
>     .call();
> 
> assert!(result.is_ok())
> ```

#### Get history
> ```rs
> use weer_api::{*, chrono::{Utc, TimeZone}};
> 
> let client = Client::new("api_key", true);
> let result = client.history()
>     .query(Query::Ip(None))
>     .dt(Utc.ymd(2022, 07, 21).and_hms(0, 0, 0))
>     .hour()
>     .call();
> 
> assert!(result.is_ok())
> ```

## License
- [MIT](LICENSE)
