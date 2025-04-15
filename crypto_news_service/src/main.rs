use axum::{extract::Query, response::Html, routing::get, Router};
use hyper::Server;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr};
use tokio;
use reqwest;
use serde_json::Value;
use chrono::{DateTime, Utc};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search_news));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn index() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Crypto News Search</title>
        <style>
            body { font-family: sans-serif; margin: 2em; }
            .news { margin-top: 1.5em; }
        </style>
    </head>
    <body>
        <h1>Search Crypto News</h1>
        <form method="get" action="/search">
            <input type="text" name="query" placeholder="e.g. BTC or Bitcoin" required>
            <button type="submit">Search</button>
        </form>
    </body>
    </html>
    "#)
}

#[derive(Deserialize)]
struct SearchParams {
    query: String,
}

async fn search_news(Query(params): Query<SearchParams>) -> Html<String> {
    let mut html = format!("<h1>Results for '{}'</h1>", params.query);

    // Simulate fetching from CryptoNews API
    let news = fetch_dummy_news(&params.query).await;
    if news.is_empty() {
        html.push_str("<p>No results found or API error occurred.</p>");
    }

    for article in news {
        html.push_str(&format!(
            "<div class='news'><h3><a href='{}'>{}</a></h3><p><strong>Source:</strong> {}</p><p><strong>Date:</strong> {}</p><p>{}</p></div>",
            article.url, article.title, article.source, article.date, article.summary
        ));
    }

    Html(format!(
        r#"<!DOCTYPE html>
        <html><head><title>News</title></head><body>{}<br><a href="/">Search again</a></body></html>"#,
        html
    ))
}

struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    url: String,
}

async fn fetch_dummy_news(query: &str) -> Vec<NewsArticle> {
    let api_key = "pub_80591b8e375b38437c7776422624f686ce812"; // TODO: replace with your key
    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&category=business&language=en",
        api_key,
        query
    );

    let response = reqwest::get(&url).await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("API error: {}", resp.status());
                return vec![];
            }

            let json = match resp.json::<Value>().await {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("JSON parse error: {:?}", e);
                    return vec![];
                }
            };

            let mut results = Vec::new();

            if let Some(articles) = json["results"].as_array() {
                for article in articles.iter().take(5) {
                    results.push(NewsArticle {
                        title: article["title"].as_str().unwrap_or("No title").to_string(),
                        source: article["source_id"].as_str().unwrap_or("Unknown").to_string(),
                        date: article["pubDate"].as_str().unwrap_or("").to_string(),
                        summary: article["description"].as_str().unwrap_or("").to_string(),
                        url: article["link"].as_str().unwrap_or("#").to_string(),
                    });
                }
            } else {
                eprintln!("No articles found.");
            }

            results
        }
        Err(e) => {
            eprintln!("Failed to fetch news: {:?}", e);
            vec![]
        }
    }
}

