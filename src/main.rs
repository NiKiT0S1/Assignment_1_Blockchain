use axum::{routing::get, Router, extract::Query, response::Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::net::SocketAddr;
use std::collections::HashMap;
use tower_http::cors::{CorsLayer, Any};
use tokio::io::AsyncReadExt;
use axum::response::Html;
use tokio::fs::File;

#[derive(Deserialize)]
struct NewsQuery {
    name: String,    // Заменяем symbol на name для поиска по названию
    page: Option<u32>,   // Параметр для страницы
    limit: Option<u32>,  // Параметр для лимита (количества элементов на странице)
    filter: Option<String>,  // Фильтрация по ключевому слову в описании
}

#[derive(Serialize, Deserialize, Debug)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    link: String,
    image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct CryptoNewsResponse {
    data: Vec<CryptoNewsArticle>,
}

#[derive(Deserialize, Debug)]
struct CryptoNewsArticle {
    news_url: String,
    image_url: Option<String>,
    title: String,
    text: String,
    source_name: String,
    date: String,
    topics: Vec<String>,
    sentiment: String,
    tickers: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CoinMarketCapResponse {
    data: HashMap<String, CoinData>,
}

#[derive(Deserialize, Debug)]
struct CoinData {
    name: String,
    description: String,
    urls: CoinUrls,
}

#[derive(Deserialize, Debug)]
struct CoinUrls {
    website: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CoinGeckoResponse {
    id: String,
    name: String,
    symbol: String,
    description: HashMap<String, String>,
    current_price: Option<HashMap<String, f64>>,
}

async fn fetch_news(Query(params): Query<NewsQuery>) -> Result<Json<Vec<NewsArticle>>, String> {
    let client = Client::new();
    let mut articles = vec![];

    // ========== CoinMarketCap ==========
    let api_key = "34a1669c-063f-41f1-bf25-56f818954268";
    let cmc_url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/info?slug={}",
        params.name.to_lowercase()
    );

    if let Ok(response) = client.get(&cmc_url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send().await {
        if let Ok(parsed) = response.json::<CoinMarketCapResponse>().await {
            if let Some(data_cmc) = parsed.data.values().find(|coin| coin.name.to_lowercase() == params.name.to_lowercase()) {
                let article_cmc = NewsArticle {
                    title: data_cmc.name.clone(),
                    source: "CoinMarketCap".to_string(),
                    date: "2025-04-02".to_string(),
                    summary: data_cmc.description.clone(),
                    link: data_cmc.urls.website.get(0).unwrap_or(&"#".to_string()).clone(),
                    image_url: None,
                };
                articles.push(article_cmc);
            } else {
                println!("CoinMarketCap: Coin not found for {}", params.name);
            }
        } else {
            println!("Failed to parse CoinMarketCap response");
        }
    } else {
        println!("Failed to fetch from CoinMarketCap");
    }

    // ========== CoinGecko ==========
    let gecko_url = format!("https://api.coingecko.com/api/v3/coins/{}", params.name.to_lowercase());

    if let Ok(response) = client.get(&gecko_url).send().await {
        if let Ok(parsed) = response.json::<CoinGeckoResponse>().await {
            let article_gecko = NewsArticle {
                title: parsed.name.clone(),
                source: "CoinGecko".to_string(),
                date: "2025-04-02".to_string(),
                summary: parsed.description.get("en").unwrap_or(&"No description available".to_string()).clone(),
                link: format!("https://www.coingecko.com/en/coins/{}", parsed.id),
                image_url: None,
            };
            articles.push(article_gecko);
        } else {
            println!("Failed to parse CoinGecko response");
        }
    } else {
        println!("Failed to fetch from CoinGecko");
    }    

    // ========== CryptoNews ==========
    let news_url = format!(
        "https://cryptonews-api.com/api/v1?tickers={}&items=3&page=1&token=61k3ttrmkkfslwqrzb77xngxldjwxfnvb7gud6b7",
        params.name.to_uppercase()
    );

    if let Ok(response) = client.get(&news_url).send().await {
        if let Ok(parsed) = response.json::<CryptoNewsResponse>().await {
            let mut crypto_news_articles: Vec<NewsArticle> = parsed.data.into_iter().map(|news| NewsArticle {
                title: news.title.clone(),
                source: news.source_name.clone(),
                date: news.date.clone(),
                summary: news.text.clone(),
                link: news.news_url.clone(),
                image_url: news.image_url,
            }).collect();
            articles.append(&mut crypto_news_articles);
        } else {
            println!("Failed to parse CryptoNews API response");
        }
    } else {
        println!("Failed to fetch from CryptoNews API");
    }

    // ========== Optional Filtering ==========
    if let Some(ref filter) = params.filter {
        articles.retain(|article| article.summary.contains(filter));
    }

    // ========== Optional Pagination ==========
    if let Some(limit) = params.limit {
        articles.truncate(limit as usize);
    }

    Ok(Json(articles))
}

async fn serve_front() -> Result<Html<String>, String> {
    let file_path = "src/front.html";
    let mut file = File::open(file_path).await.map_err(|e| format!("Failed to read file: {}", e))?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)
        .await
        .map_err(|e| format!("Failed to read file content: {}", e))?;

    Ok(Html(content))  // Отправляем содержимое HTML файла в ответе
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:5000".parse().expect("Invalid address format");
    
    // Настройка CORS для разрешения запросов с любого источника
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/", get(serve_front))  // Маршрут для отдачи front.html по умолчанию
        .route("/news", get(fetch_news))
        .layer(cors); // Добавляем слой CORS
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    println!("Server is running at http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
