use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod api;

pub use anyhow::Result;
pub use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug)]
pub struct City {
  pub city: String,
  pub state_id: String,
  pub lat: f32,
  pub lng: f32,
}

impl City {
  pub fn into_point(&self) -> Result<Point> {
    Ok(Point {
      lat: self.lat,
      lng: self.lng,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
  pub lat: f32,
  pub lng: f32,
}

impl Point {
  // create a new 'Point' from (f32, f32)
  pub fn new(lat: &f32, lng: &f32) -> Result<Self> {
    Ok(Point {
      lat: *lat,
      lng: *lng,
    })
  }
}

// result of a point query
#[derive(Serialize, Deserialize, Debug)]
pub struct PointInfo {
  id: String,
  pub properties: PointProps,
}

// inner properties object of PointInfo
#[derive(Serialize, Deserialize, Debug)]
pub struct PointProps {
  #[serde(rename(deserialize = "forecastOffice"))]
  pub forecast_office: String,
  pub forecast: String,
  #[serde(rename(deserialize = "forecastHourly"))]
  pub forecast_hourly: String,
  #[serde(rename(deserialize = "forecastGridData"))]
  pub forecast_grid_data: String,
  #[serde(rename(deserialize = "observationStations"))]
  pub observation_stations: String,
  #[serde(rename(deserialize = "relativeLocation"))]
  pub relative_location: RelativeLocation,
  #[serde(rename(deserialize = "forecastZone"))]
  pub forecast_zone: String,
  pub county: String,
  #[serde(rename(deserialize = "fireWeatherZone"))]
  pub fire_weather_zone: String,
  #[serde(rename(deserialize = "timeZone"))]
  pub time_zone: String,
  #[serde(rename(deserialize = "radarStation"))]
  pub radar_station: String,
}

// inner relative_location object of PointProps
#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeLocation {
  pub geometry: Value,
  pub properties: RelativeProps,
}

// inner properties object of RelativeLocation
#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeProps {
  pub city: String,
  pub state: String,
  pub distance: Value,
  pub bearing: Value,
}

/// Result of Forecast query
#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
  pub properties: ForecastProps,
}

// Inner properties object of Forecast
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastProps {
  pub updated: DateTime<Utc>,
  pub units: String,
  #[serde(rename(deserialize = "generatedAt"))]
  pub generated_at: DateTime<Utc>,
  pub elevation: Value,
  pub periods: Vec<ForecastPeriod>,
}

/// Single instance of item in periods object of ForecastProps
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastPeriod {
  pub number: u16,
  pub name: String,
  #[serde(rename(deserialize = "startTime"))]
  pub start_time: DateTime<Utc>,
  #[serde(rename(deserialize = "endTime"))]
  pub end_time: DateTime<Utc>,
  #[serde(rename(deserialize = "isDaytime"))]
  pub is_day_time: bool,
  pub temperature: i8,
  #[serde(rename(deserialize = "temperatureUnit"))]
  pub temperature_unit: String,
  #[serde(rename(deserialize = "windSpeed"))]
  pub wind_speed: String,
  #[serde(rename(deserialize = "windDirection"))]
  pub wind_direction: String,
  pub icon: String,
  #[serde(rename(deserialize = "shortForecast"))]
  pub short_forecast: String,
  #[serde(rename(deserialize = "detailedForecast"))]
  pub detailed_forecast: String,
}

/// Forecast output representation
#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastBundle {
  pub start: DateTime<Utc>,
  pub end: DateTime<Utc>,
  pub temperature: i8,
  pub wind_speed: String, // TODO parse from string to int "30 mph" -> 30
  pub wind_direction: String,
  pub short_forecast: String,
}

/// Weather output representation
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherBundle {
  pub location: City,
  pub forecast: Vec<ForecastBundle>,
  pub updated: DateTime<Utc>,
}

impl WeatherBundle {
  pub fn new(loc: City, fcb: Forecast) -> Self {
    let mut vec = Vec::new();
    for i in fcb.properties.periods.iter() {
      let i = ForecastBundle {
        start: i.start_time,
        end: i.end_time,
        temperature: i.temperature,
        wind_speed: i.wind_speed.to_string(),
        wind_direction: i.wind_direction.to_string(),
        short_forecast: i.short_forecast.to_string(),
      };
      vec.push(i);
    }
    WeatherBundle {
      location: loc,
      forecast: vec,
      updated: fcb.properties.updated,
    }
  }
}
