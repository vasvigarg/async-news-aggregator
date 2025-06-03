use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔄 Fetching latest news...\n");

    let client = Client::new();

    let tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = vec![
        Box::pin(fetch_the_hindu(client.clone())),
        Box::pin(fetch_livemint(client.clone())),
    ];

    join_all(tasks).await;

    println!("🥳 All news fetched successfully. Stay informed!\n");
    Ok(())
}

async fn fetch_the_hindu(client: Client) {
    println!("✏️ LATEST NEWS HEADLINES FROM THE HINDU:\n");

    match client.get("https://www.thehindu.com").send().await {
        Ok(resp) => match resp.text().await {
            Ok(body) => {
                let document = Html::parse_document(&body);
                // Using only the specified selector for The Hindu
                let selector = Selector::parse("h3.title a").unwrap(); 

                let mut count = 0;
                for el in document.select(&selector) {
                    let title = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    let mut link = el.value().attr("href").unwrap_or("#").to_string();

                    if !link.starts_with("http") && link != "#" {
                        link = format!("https://www.thehindu.com{}", link);
                    }

                    if !title.is_empty() && link != "https://www.thehindu.com#" { 
                        println!("📰 {title}\n🔗 {link}\n");
                        count += 1;
                        if count >= 5 {
                            break;
                        }
                    }
                }

                if count == 0 {
                    println!("⚠️ Could not find any headlines from The Hindu.\n");
                }
            }
            Err(_) => println!("⚠️ Failed to parse The Hindu homepage.\n"),
        },
        Err(_) => println!("⚠️ Failed to fetch The Hindu homepage.\n"),
    }
    println!("\n");
}

async fn fetch_livemint(client: Client) {
    println!("✏️ LATEST NEWS HEADLINES FROM LIVEMINT:\n");

    match client.get("https://www.livemint.com/").send().await {
        Ok(resp) => match resp.text().await {
            Ok(body) => {
                let document = Html::parse_document(&body);
                // Using only the specified selector for Livemint
                let selector = Selector::parse("h3.imgStory a").unwrap(); 

                let mut count = 0;
                for el in document.select(&selector) {
                    let title = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    let link = el.value().attr("href").unwrap_or("#");

                    let full_link = if !link.starts_with("http") && link != "#" {
                        format!("https://www.livemint.com{}", link)
                    } else {
                        link.to_string()
                    };
                    
                    if !title.is_empty() && full_link != "https://www.livemint.com#" && full_link != "#" {
                        println!("📰 {title}\n🔗 {full_link}\n");
                        count += 1;
                        if count >= 5 {
                            break;
                        }
                    }
                }

                if count == 0 {
                    println!("⚠️ Could not find any headlines from Livemint.\n");
                }
            }
            Err(_) => println!("⚠️ Failed to parse Livemint homepage.\n"),
        },
        Err(_) => println!("⚠️ Failed to fetch Livemint homepage.\n"),
    }
    println!("\n");
}
