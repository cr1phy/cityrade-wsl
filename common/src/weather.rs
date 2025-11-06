#[derive(Debug, Default)]
pub struct Weather {
    condition: WeatherCondition,
    temperature: f64,
    humidity: f32,
    wind_speed: f64,
    wind_direction: f32,
    pressure: f32,
    precipitation: f32,
}

#[derive(Debug, Default)]
pub enum WeatherCondition {
    #[default]
    Unknown,
    Sunny,
    Cloudy,
    PartlyCloudy,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
    Windy,
}

#[derive(Debug, Default)]
pub enum Season {
    #[default]
    Unknown,
    Spring,
    Summer,
    Autumn,
    Winter,
}
