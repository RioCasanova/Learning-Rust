use reqwest::Client;

    
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
*/
    
    
    
    // what is HTTP?

    // HTTP stands for Hypertext Transfer Protocol
    // and structures requests and responses for transfers over the internet.
    // It does this using TCP, which stands for Transmission Control Protocol.
    // TCP allows us to make a connection to the server 
    // TCP manages the channels between our browser and the server, used to manage many types of
    // internet connections in which one computer wants to send something to another computer.
    // HTTP are the rules that the clients must abide by to transfer information over the internet

    // Once the client establishes a TCP connection to the server, our browser to client
    // sends a 'get' request to the server asking for some information, such as a website or webpage
    // after the server returns a response, the TCP connection is closed.

    // When searching on a browser, the browser will see the HTTP or HTTPS part of the 
    // url and recognize it as the protocol to use.
    // Then it will take the domain name from the url, and ask the domain name server
    // to return a internet protocol address or IP Address for the domain name. 
    // Now the browser knows where to find the webpage we are looking for and we open a
    // connection to the server at that address using HTTP protocol specified.
    // we then get a response from the server, either that there were not issues getting the 
    // website (200) or with a blank page and a 404 not found code.

#[tokio::main]
async fn main() {
    // Client: Can send multiple requests - can configure timeouts/connection pooling/proxys
    let client = Client::new();

    // Using the 'get' Client method will take in the url and return a 
    // customizable RequestBuilder type that allows us to specify what we would 
    // like to include in the headers, body and other http request fields
    let response = client.get("https://scrapeme.live/shop/")
        .send() // this is a method on the RequestBuilder type that returns either a response or an error
        
        // it actually returns: impl Futue<Output = Result<Response, Error>>

        // impl Future: this means that what we return implements the Future trait. This value
            // will be available at some point but not right now, and it is known that the
            // type will be a Result type but we do not know if it will be successful or not

        // Output =: This is an associated type of the Future trait that specifies the value that 
            // the Future will evaluate to once it is resolved, in this case Result<T, E>

        // Result<Response, Error>: This will return either a Response or an Error depending on
            // if the operation was successful, it will be returned in this form Ok(Response) or Err(Error)
        
        .await // Ok(Response) or Err(Error)
        .unwrap(); // we are using unwrap here to access the value that is returned Ok(Response) or Err(Error)

        // When we get here, 'response' is of type reqwest::Response
        // and we are calling the 'text' method on it which returns Result<String> or Ok(String)
        // Because it is a 'Result' type we need to 'unwrap' it.
        // this will return the webpage as a very long string.
        // meaning that it is being stored on the heap and it is owned.

        // You may be wondering why we are calling await on the response method when we 
        // know that at this point the response would be known.
        // This is because the method that is being called is an 'async fn', which is just 
        // another way of saying that the function immediate returns a Future and does not
        // execute until 'await' is called - that is why you may hear people refer to 
        // async/await functionality in rust 'lazy'.
    let html = response.text().await.unwrap();

    // After scraping the website of its html data we call 'parse_document'
    // to turn it into a struct that we can then pull apart and actually get data to
    // for example instead of trying to find where the string contains a li
    // we can use the methods provided by the 'scraper' crate
    // Think of this as formatting so we can use the data
    let document = scraper::Html::parse_document(&html);

    // Here we are creating a selector that takes in an element and its class
    // and returns a result type (so we unwrap) and now the selector contains 
    // a portion of the html that we wish to extract further

    // This creates a Selector object for the "li.product" CSS selector.
// It is used to find all `<li>` elements with the class "product" in the HTML document.

    // So here we are saying that we want to get all the elements with this tag and class
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();

    // This returns a 'Select' type which in this case is an iterator
    // This creates an iterator over all elements in the document that match 
    // the "li.product" selector.
    // `document.select` uses the selector to extract these elements, which we can iterate over
    // to access their data.
    let html_products = document.select(&html_product_selector);

    // product is of type 'ElementRef' which comes with a bunch of handy methods to
    // access the attributes and inner pieces of each element within our iterable
    for product in html_products {


        let url = product  

            // This here is doing the same as before with the 'select' method
            // We are saying that we want to select all the 'a' elements within the 'product' element
            .select(&scraper::Selector::parse("a").unwrap())

            // `.next()` retrieves the first `<a>` element in the iterator, if it exists.
            .next()
            
             // `and_then` accesses the `<a>` element's attributes.
            // `.value()` gives access to the raw element data.
            // `.attr("href")` retrieves the value of the "href" attribute, which is the URL.
            .and_then(|a| a.value().attr("href"))

            // If the "href" attribute exists, `str::to_owned` converts the borrowed string
            // slice (&str) into an owned String.
            .map(str::to_owned)

            .unwrap(); // we unwrap here because it is an option or result type

        let image_url = product  
            .select(&scraper::Selector::parse("img").unwrap())
            .next().and_then(|a| a.value().attr("src"))
            .map(str::to_owned)
            .unwrap();


        let product_name = product  
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>())
            .unwrap();
        
        let product_price = product  
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            // The reason these two last ones are different is because we are accessing 
            // text variables instead of extracting data right from the attributes (href, src)
            // we call 'collect' because the 'text' method returns an iterator.
            // but we want a single value not a deconstructed one, so we call collect
            // and consume the iteration and turn it into a string.
            .map(|h2| h2.text().collect::<String>())
            .unwrap();

        // Here we are just printing using debug notation for our data
        println!("name: {:?} price: {:?} url: {:?} image_path: {:?}", product_name,product_price, url, image_url)
    }
}
