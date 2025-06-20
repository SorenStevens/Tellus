use reqwest::Client;
use serde::Deserialize;
use anyhow::{ Result, anyhow };

/* JSON PARSERS */
/* Location Data */
#[derive(Debug, Deserialize)]
pub struct LocationResponse {
    pub properties: LocationProperties,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationProperties {
    forecast: String,
    forecast_office: String,
    relative_location: RelativeLocationWrapper,
}

#[derive(Debug, Deserialize)]
pub struct RelativeLocationWrapper {
    pub properties: RelativeLocation,
}

#[derive(Debug, Deserialize)]
pub struct RelativeLocation {
    city: String,
    state: String,
}


/* Forecast Data */
#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    properties: ForecastProperties,
}

#[derive(Debug, Deserialize)]
pub struct ForecastProperties {
    periods: Vec<ForecastPeriod> // from 1 - 14
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPeriod {
    name: String, // E.x. Today
    detailed_forecast: String, // Weather report
}


/* Alert Data */
#[derive(Debug, Deserialize)]
pub struct AlertResponse {
    #[serde(default)]
    features: Vec<AlertFeature>,
}

#[derive(Debug, Deserialize)]
pub struct AlertFeature {
    properties: AlertProperties,
}

#[derive(Debug, Deserialize)]
pub struct AlertProperties {
    headline: String,
}

/* API Callers */
pub async fn fetch_location_data(client: &Client, latitude: &f64, longitude: &f64) -> anyhow::Result<LocationResponse> {
    let location_url = format!("https://api.weather.gov/points/{},{}", latitude, longitude);
    Ok(client.get(location_url).send().await?.json::<LocationResponse>().await?)
}

pub async fn fetch_forecast_data(client: &Client, url: &String) -> anyhow::Result<ForecastResponse> {
    Ok(client.get(url).send().await?.json::<ForecastResponse>().await?)
}

pub async fn fetch_alert_data(client: &Client, latitude: &f64, longitude: &f64) -> anyhow::Result<AlertResponse> {
    let alert_url = format!("https://api.weather.gov/alerts/active?point={},{}", latitude, longitude);
    Ok(client.get(alert_url).send().await?.json::<AlertResponse>().await?)
}