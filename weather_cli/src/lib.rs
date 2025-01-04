pub mod weather {
    // imports
    use colored::*;
    use serde::Deserialize;
    
    // struct to deserialize the JSON response from the openweatherMap API
    #[derive(Debug, Deserialize)]
    pub struct WeatherResponse {
        weather: Vec<Weather>,
        main: Main,
        wind: Wind,
        name: String,
    }

    // struct to represent the weather description
    #[derive(Debug, Deserialize)]
    pub struct Weather {
        description: String,
    }

    // struct to represent the weather parameters
    #[derive(Debug, Deserialize)]
    pub struct Main {
        temp: f64,
        humidity: f64,
        pressure: f64,
    }

    // struct to represent the wind information
    #[derive(Debug, Deserialize)]
    pub struct Wind {
        speed: f64,
    }

    // function to get the weather information from the openweathermap api

    pub fn get_weather(
        city: &str,
        country_code: &str,
        api_key: &str,
    ) -> Result<WeatherResponse, reqwest::Error> {
        let url: String = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city, country_code, api_key
        );
        let response = reqwest::blocking::get(&url)?;
        let res_json: WeatherResponse = response.json::<WeatherResponse>()?;
        Ok(res_json)
    }

    // Function to display the weather information
    pub fn display_weather_info(response: &WeatherResponse) {
        // Extract weather information from the response
        let description: &String = &response.weather[0].description;
        let temperature: f64 = response.main.temp;
        let humidity: f64 = response.main.humidity;
        let pressure: f64 = response.main.pressure;
        let wind_speed: f64 = response.wind.speed;

        // Formatting it all into a string for output
        let weather_text: String = format!(
            "Weather in {}: {} {} 
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hP
        > Wind Speed: {:.1} m/s",
            response.name,
            description,
            get_emoji(temperature),
            temperature,
            humidity,
            pressure,
            wind_speed
        );

        // Colouring the weather text based on weather condition
        let weather_text_colour: ColoredString = match description.as_str() {
            "clear sky" => weather_text.bright_yellow(),
            "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
            "dust" | "fog" | "squalls" | "smoke" | "sand" | "mist" | "haze" | "overcast clouds" => {
                weather_text.dimmed()
            }
            "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
            _ => weather_text.normal(),
        };

        println!("{}", weather_text_colour);
    }

    // function to get the emoji based on the temp
    pub fn get_emoji(temperature: f64) -> &'static str {
        if temperature < 0.0 {
            "❄️"
        } else if temperature >= 0.0 && temperature < 10.0 {
            "☁️"
        } else if temperature >= 10.0 && temperature < 20.0 {
            "🌤️"
        } else {
            "🔥"
        }
    }
}
