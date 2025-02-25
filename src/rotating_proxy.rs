use reqwest;
use std::error::Error;
use rand::seq::SliceRandom;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{
    let url = "http://books.toscrape.com/";

    let proxies = get_proxies();
    let random_proxy = proxies.choose(&mut rand::thread_rng()).unwrap();

    let client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::http(format!("{0}:{1}", random_proxy.ip, random_proxy.port))?)
    .build()?;

    let response = client
        .get(url)
        .send()
        .await?;

    let html_content = response.text().await?;

    extract_products(&html_content);

    Ok(())
}

fn extract_products(html_content: &str) {

    let document = scraper::Html::parse_document(&html_content);

    let html_product_selector = scraper::Selector::parse("article.product_pod").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut products: Vec<Product> = Vec::new();

    for html_product in html_products {
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&scraper::Selector::parse("h3").unwrap())
            .next()
            .map(|title| title.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price_color").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        let product = Product {
            url,
            image,
            name,
            price,
        };
        products.push(product);
    }

    println!("{:?}", products);
}

#[derive(Debug)]
struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

#[derive(Debug)]
struct Proxy {
    ip: String,
    port: String,
}

fn get_proxies() -> Vec<Proxy> {
    let mut proxies = Vec::new();

    proxies.push(Proxy {
        ip: "http://localhost".to_string(),
        port: "8080".to_string(),
    });

    // Add more proxies.push statements here to create a bigger set of proxies

    proxies
}
