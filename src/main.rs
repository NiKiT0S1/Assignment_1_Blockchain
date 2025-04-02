use axum::{routing::get, Router, extract::Query, response::Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::collections::HashMap;

#[derive(Deserialize)]
struct NewsQuery {
    symbol: String,
    page: Option<u32>,   // Параметр для страницы
    limit: Option<u32>,  // Параметр для лимита (количества элементов на странице)
    filter: Option<String>,  // Фильтрация по ключевому слову в описании
}

#[derive(Serialize, Deserialize)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    link: String,
}

#[derive(Deserialize)]
struct CoinMarketCapResponse {
    data: HashMap<String, CoinData>,
}

#[derive(Deserialize)]
struct CoinData {
    name: String,
    description: String,
    urls: CoinUrls,
}

#[derive(Deserialize)]
struct CoinUrls {
    website: Vec<String>,
}

async fn fetch_news(Query(params): Query<NewsQuery>) -> Result<Json<Vec<NewsArticle>>, String> {
    let api_key = "34a1669c-063f-41f1-bf25-56f818954268"; // Убедись, что ключ актуален.
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/info?symbol={}",
        params.symbol
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?
        .json::<CoinMarketCapResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let data = response.data.get(&params.symbol).ok_or_else(|| "No data found for symbol".to_string())?;

    // Применение фильтрации
    let mut article = NewsArticle {
        title: data.name.clone(),
        source: "CoinMarketCap".to_string(),
        date: "2025-04-02".to_string(),
        summary: data.description.clone(),
        link: data.urls.website.get(0).unwrap_or(&"#".to_string()).clone(),
    };

    // Если задан параметр filter, то фильтруем по описанию (summary)
    if let Some(ref filter) = params.filter {
        if !article.summary.contains(filter) {
            return Ok(Json(Vec::new()));  // Возвращаем пустой список, если фильтрация не проходит
        }
    }

    // Пагинация: Здесь можно ограничить количество возвращаемых статей.
    let articles = vec![article];  // Пока одна статья на примере, можно добавить пагинацию для нескольких статей

    Ok(Json(articles))
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:5000".parse().expect("Invalid address format");

    let app = Router::new().route("/news", get(fetch_news));

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    println!("Server is running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}