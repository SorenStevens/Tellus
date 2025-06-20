use std::env;
use serde_json;

use rig::{ agent::AgentBuilder, completion::Prompt, providers::openai };

include!("api_call.rs");

async fn get_lat_lon(client: &Client, city: &str, state: &str) -> Result<(f64, f64)> {
    let url = format!("https://nominatim.openstreetmap.org/search?city={}&state={}&country=USA&format=json&limit=1", city, state);
    let response = client.get(&url).send().await?.json::<Vec<serde_json::Value>>().await?;
    if let Some(entry) = response.get(0) {
        let lat = entry["lat"].as_str().ok_or_else(|| anyhow!("No Latitude"))?.parse::<f64>()?;
        let lon = entry["lon"].as_str().ok_or_else(|| anyhow!("No Longitude"))?.parse::<f64>()?;
        Ok((lat, lon))
    } else {
        Err(anyhow!("No results from geocoding API"))
    }
}

pub async fn prompt_model(city: String, state: String) -> Result<()>{
    /* Build Client for NWS API */
    let client = reqwest::Client::builder().user_agent("UrgencyWeatherAgent/1.0DATAPROCESSOR (Contact: soren@triplenexus.org)").build()?;
    
    println!("-- Getting Geo Coordinates --\n");
    let (latitude, longitude) = get_lat_lon(&client, &city, &state).await?;
    
    println!("-- Fetching Data From Nearest NWS Station --\n");
    println!("* Nearest NWS Station May be in A Different City *\n");
    
    /* Call NWS (lat,lon) API */
    let location_response = fetch_location_data(&client, &latitude, &longitude).await?;
    
    /* Unpack JSON API response for location call */
    let city = &location_response.properties.relative_location.properties.city;
    let state = &location_response.properties.relative_location.properties.state;
    let forecast_office = &location_response.properties.forecast_office;
    
    /* Call NWS (forecast) API */
    let forecast_response = fetch_forecast_data(&client, &location_response.properties.forecast).await?;
    
    /* Unpack JSON API response for weather call */        
    let todays_forecast = forecast_response.properties.periods.get(0)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    let forecast_one = forecast_response.properties.periods.get(1)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    let forecast_two = forecast_response.properties.periods.get(2)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    let forecast_three = forecast_response.properties.periods.get(3)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    let forecast_four = forecast_response.properties.periods.get(4)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    let forecast_five = forecast_response.properties.periods.get(5)
        .map(|p| format!("{}, {}", p.name, p.detailed_forecast))
        .unwrap_or("No data found".to_string());
    
    /* Agent */
    let openai_client = openai::Client::new(
        &env::var("OPENAI_API_KEY")
        .expect("Enviornment Variable OPENAI_API_KEY Not Set"));
    let model = openai_client.completion_model("gpt-4o");
    let agent = AgentBuilder::new(model)
        .context("Don't ask follow up questions.")
        .context("You are an insightful weather analyst, tasked with giving multiple days of forecast.")
        .context("You receive data being injected from the National Weather Service API.")
        .context("You have a focus on being as functional and analytical as possible with the data you are given.")
        .context("You ues ASCII formatting to make labeled data tables of the data you are given.")
        .context("You will receive 5 total entries. The data is divided by new line characters. The first entry is the current forecast.")
        .context("IMPORTANT: Ensure that all 5 entries are included in the table")
        .context("When you are asked about weather alerts, please be sure to inform the user if there are none.")
        .context("If there are weather alerts, they should be the main focus of your response.")
        .context("You have a focus on taking the next forecasted days to give the user an analysis of what weather patterns around this time of year are like.")
        .context(&format!("This is a weather report for {}, {}.", city, state))
        .context(&format!("The nearest National Weather Service weather station is {}.", forecast_office))
        .context(&format!("Here is today's data from the National Weather Service API: {}.", todays_forecast))
        .context(&format!("Here is the next entry from the National Weather Serivce API: {}.", forecast_one))
        .context(&format!("Here is the next entry from the National Weather Serivce API: {}.", forecast_two))
        .context(&format!("Here is the next entry from the National Weather Serivce API: {}.", forecast_three))
        .context(&format!("Here is the next entry from the National Weather Serivce API: {}.", forecast_four))
        .context(&format!("Here is the next entry from the National Weather Serivce API: {}.", forecast_five))
        .context("here is an example header to your output:
            Weather Report for Hoboken, NJ:
            ------------------------------------------------
            | Time                | Conditions             |
            ------------------------------------------------
            | Overnight           |                         |
            | Current Forecast    | Partly cloudy           |
            | Temperature         | Low around 57°F         |
            | Wind                | West wind around 3 mph  |
            ------------------------------------------------
            | Tomorrow Morning    |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | Expected rise to 59°F   |
            | Wind                | West wind around 3 mph  |
            ------------------------------------------------
            | Tomorrow Evening    |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | Low around 57°F         |
            | Wind                | West wind around 3 mph  |
            ------------------------------------------------
            | Following Day       |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | High near 70°F, Low 55°F|
            | Wind                | Variable winds          |
            ------------------------------------------------
            | Following Evening   |                         |
            | Conditions          | Mostly clear            |
            | Temperature         | High near 68°F, Low 54°F|
            | Wind                | Calm wind prevailing    |
            ------------------------------------------------
            ")
        .context("Here is another example:
            Weather Report for Chicago, IL:
            ------------------------------------------------
            | Time                | Conditions              |
            ------------------------------------------------
            | Tonight             |                         |
            | Conditions          | Cloudy                  |
            | Temperature         | Low around 50°F         |
            | Wind                | East northeast 5-10 mph |
            ------------------------------------------------
            | Tomorrow            |                         |
            | Conditions          | Mostly sunny            |
            | Temperature         | High near 70°F, Low 52°F|
            | Wind                | east wind 5-10 mph      |
            ------------------------------------------------
            | Tomorrow Evening    |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | Low 54°F                |
            | Wind                | Southeast 5 mph         |
            ------------------------------------------------
            | Following Day       |                         |
            | Conditions          | Sunny                   |
            | Temperature         | High near 72°F, Low 53°F|
            | Wind                | South southeast 5-10 mph|
            ------------------------------------------------
            | Following Evening   |                         |
            | Conditions          | Clear                   |
            | Temperature         | Low 55°F                |
            | Wind                | Calm wind               |
            ------------------------------------------------
            | Third Day           |                         |
            | Conditions          | Sunny                   |
            | Temperature         | High near 74°F, Low 56°F|
            | Wind                | Southwest 5-10 mph      |
            ------------------------------------------------
            | Third Evening       |                         |
            | Conditions          | Mostly clear            |
            | Temperature         | Low 57°F                |
            | Wind                | West wind around 5 mph  |
            ------------------------------------------------
            | Fourth Day          |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | High near 75°F, Low 58°F|
            | Wind                | North northwest 5 mph   |
            ------------------------------------------------
            | Fourth Evening      |                         |
            | Conditions          | Partly cloudy           |
            | Temperature         | Low 59°F                |
            | Wind                | East northeast 5 mph    |
            ------------------------------------------------
            | Fifth Day           |                         |
            | Conditions          | Mostly sunny            |
            | Temperature         | High near 76°F, Low 60°F|
            | Wind                | East 5-10 mph           |
            ------------------------------------------------
            ")
        .context("Here is another example:
            Weather Report for Fraser, CO:
            ```
            ----------------------------------------------------
            | Time                | Conditions                  |
            ----------------------------------------------------
            | Overnight           |                             |
            | Conditions          | Mostly clear                |
            | Temperature         | Low around 37°F             |
            | Wind                | South southwest 5 mph       |
            ----------------------------------------------------
            | Current Forecast    |                             |
            | Conditions          | Mostly clear                |
            | Temperature         | Low around 37°F             |
            | Wind                | South southwest 5 mph       |
            ----------------------------------------------------
            | Following Morning   |                             |
            | Conditions          | Clear                       |
            | Temperature         | High near 55°F              |
            | Wind                | South 5 to 10 mph           |
            ----------------------------------------------------
            | Following Evening   |                             |
            | Conditions          | Partly cloudy               |
            | Temperature         | Low around 34°F             |
            | Wind                | Southeast wind around 5 mph |
            ----------------------------------------------------
            | Third Day           |                             |
            | Conditions          | Sunny                       |
            | Temperature         | High near 60°F, Low 35°F    |
            | Wind                | West southwest wind 5-10 mph|
            ----------------------------------------------------
            | Third Evening       |                             |
            | Conditions          | Partly cloudy               |
            | Temperature         | Low around 32°F             |
            | Wind                | Calm wind                   |
            ----------------------------------------------------
            | Fourth Day          |                             |
            | Conditions          | Mostly sunny                |
            | Temperature         | High near 63°F, Low 36°F    |
            | Wind                | South wind 5-10 mph         |
            ----------------------------------------------------
            | Fourth Evening      |                             |
            | Conditions          | Partly cloudy               |
            | Temperature         | Low around 33°F             |
            | Wind                | West wind around 5 mph      |
            ----------------------------------------------------
            | Fifth Day           |                             |
            | Conditions          | Sunny                       |
            | Temperature         | High near 65°F, Low 37°F    |
            | Wind                | East wind 5-10 mph          |
            ----------------------------------------------------
            ")
        .context("Ensure all weather forecasts are included in the table.")
        .build();

    println!("-- Prompting Agent --\n");
    
    /* Ask agent for summary of weather data */
    let weather_agent_response = agent.prompt("Give me a summary of the weather data you just received from the NWS API").await?;
    println!("{}", weather_agent_response);
    
    println!("-- Checking For NWS Alerts --\n");
    
    /* Call NWS (alert) API */
    let alert_response = fetch_alert_data(&client, &latitude, &longitude).await?;
    let alert = alert_response.features.get(0)
        .map(|a| a.properties.headline.clone())
        .unwrap_or("No Active Alerts or Headlines".to_string());
    
    println!("-- Re-prompting Agent --\n");
    
    /* Re-prompt agent with it's own weather report and add the nearest NWS headline */
    let final_prompt = format!("Given the following weather report and NWS headline, what are the implications of this weather event for the public? Weather report: {}, Headline: {}", weather_agent_response, alert);
    let alert_agent_response = agent.prompt(final_prompt).await?;
    println!("{}", alert_agent_response);
    
    Ok(())
}
