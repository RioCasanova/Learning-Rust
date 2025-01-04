use dotenvy::dotenv;
use std::{
    env,
    io::{self},
};
use weather_cli::weather::*;
use colored::*;



fn main() {
    dotenv().ok();

    println!("{}", "Welcome to the weather cli".bright_magenta());
    loop {
        // Reading in the country code
        println!("{}", "Please enter a country code (US, CA, AU, etc...): ");
        let mut country = String::new();
        io::stdin()
            .read_line(&mut country)
            .expect("Failed to read country input");
        let country = country.trim();

        // Reading in the city
        println!(
            "{}",
            "Please enter the name of a city within that country: "
        );
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read city input");
        let city = city.trim();

        // fetching the weather information

        let api_key = env::var("API_KEY").expect("API_KEY not set in env");

        match get_weather(city, country, &api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                eprintln!("Error {}", error);
            }
        }

        // asking if the user would like to continue
        println!(
            "{}",
            "Do you want to search for weather in another City? (yes/no): "
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}
