use std::iter::Product;

use reqwest::{Client, Error};
use robots_txt::{matcher::SimpleMatcher, Robots};
use thiserror::*;
use url::{ParseError, Url};

/*
    Author: Rio Casanova
    Created: January 5, 2025
    Version: 2

    Purpose:

        Build a better version of 'basic_web_scraper' that uses less binary code
        and is more intuitive. Meaning that it should be easier to use, and
        easier to know how to code it from memory.

        The overall purpose is to build Rust experience and skills as
        well as familiarity with async programming and other crates.

    Changes from V1:

        This version will determine whether the url can be scraped or what the
        robots.txt regulations are for our scraper is, and will either scrape
        or not scrape based on whether it is allowed.
*/

#[derive(Debug, Error)]
pub enum ScrapeError {
    #[error("Network error occurred: {0}")]
    NetworkError(#[from] Error),

    #[error("Failed to parse url: {0}")]
    UrlError(#[from] ParseError),

    #[error("HTML or Selector parsing error: {0}")]
    ParsingError(String),

    #[error("Failed to fetch robots.txt for {0}")]
    RobotsError(String),
}

async fn is_crawlable(client: &Client, urls: &[&str]) -> Result<bool, ScrapeError> {
    const USER_AGENT: &str = "*";

    for url in urls {
        let parsed_url = Url::parse(url)?;
        let domain = parsed_url.join("/robots.txt")?;
        let response = client.get(domain).send().await?;

        let robots_txt = response.text().await?;
        let robot = Robots::from_str_lossy(&robots_txt);
        let matcher = SimpleMatcher::new(&robot.choose_section(USER_AGENT).rules);

        println!("URL: {:?}, {:#?}", url, matcher);
    }
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), ScrapeError> {
    let client = Client::new();
    let urls = vec![
        "https://www.scrapethissite.com/",
        "https://books.toscrape.com/",
        "https://scrapeme.live/product-category/seed/",
        "https://www.alberta.ca/",
    ];

    let result = is_crawlable(&client, &urls).await?;

    if result == true {
        println!("The site can be crawled!");
        for url in urls {
            let response = client.get(url).send().await?;
            let html = response.text().await?;
            let document = scraper::Html::parse_document(&html);
            let html_product_selector = scraper::Selector::parse("li.product").unwrap();
            let html_products = document.select(&html_product_selector);

            for product in html_products {
                let for_string = product
                    .select(&scraper::Selector::parse("a").unwrap())
                    .next()
                    .and_then(|a| a.value().attr("href"))
                    .map(str::to_owned)
                    .unwrap();
            }
        }
    } else {
        println!("The site cannot be crawled.");
    };

    Ok(())
}
