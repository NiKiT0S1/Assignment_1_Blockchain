# Cryptocurrency News Aggregator

A robust server application that fetches real-time cryptocurrency data from the **CoinMarketCap API** and serves curated news articles through a RESTful API interface. The project currently focuses on backend functionality, efficiently retrieving and processing cryptocurrency data before delivering it as structured JSON responses.

## Features

- Real-time cryptocurrency data retrieval
- Customizable news filtering
- Pagination support
- Efficient API caching
- Currency name-based querying

## Requirements

- **Rust** (version 1.58.0 or higher) — for building and running the project
- **CoinMarketCap API key** — for accessing cryptocurrency data. Create your API key at [CoinMarketCap](https://coinmarketcap.com/api/) and store it in a `main.rs` file
- **CryptoNews API key** — for accessing cryptocurrency news data. Create your API key at [CryptoNews](https://cryptonews-api.com) and store it in a `main.rs` file

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/NiKiT0S1/Assignment_1_Blockchain.git
   cd Assignment_1_Blockchain
   ```

2. Install dependencies and build the project:
   ```bash
   cargo build
   ```

## Running the Application

1. Start the server:
   ```bash
   cargo run
   ```

2. The server will be available at `http://127.0.0.1:5000` by default

## API Usage

### Endpoint: `/news`

Retrieves news articles for a specified cryptocurrency.

#### Query Parameters

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| name      | Yes      | -       | Cryptocurrency name (e.g., bitcoin, ethereum) |
| limit     | No       | 5       | Number of articles per page |
| filter    | No       | -       | Keyword to filter articles by content |

#### Example Requests

Basic request for Bitcoin news:
```
http://127.0.0.1:5000/news?symbol=bitcoin
```

Request with pagination and filtering:
```
http://127.0.0.1:5000/news?name=ethereum&limit=10&filter=defi
```

#### Response Format

```json
[
  {
    "title": "Ethereum Price Surges Past $3,000",
    "source": "CoinMarketCap",
    "date": "2025-04-02",
    "summary": "Ethereum (ETH) has surpassed the $3,000 mark for the first time since...",
    "link": "https://ethereum.org/news/price-surge"
  },
  {
    "title": "New DeFi Protocol Launches on Ethereum",
    "source": "Crypto Daily",
    "date": "2025-04-01",
    "summary": "A new decentralized finance protocol has launched on the Ethereum blockchain...",
    "link": "https://cryptodaily.co.uk/ethereum-defi-launch"
  }
]
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Request successful
- `400 Bad Request`: Missing required parameters or invalid input
- `404 Not Found`: No news found for the specified cryptocurrency
- `429 Too Many Requests`: Rate limit exceeded (from CoinMarketCap API)
- `500 Internal Server Error`: Server-side error

## Development

### Project Structure

```
Assignment_1_Blockchain/
├── src/
│   ├── main.rs          # Application entry point
│   ├── api/             # API integration modules
│   ├── models/          # Data structures
│   └── handlers/        # Request handlers
├── Cargo.toml           # Project dependencies
├── Cargo.lock           # Locked dependencies
└── .env                 # Environment variables (not in repo)
```

### Running Tests

```bash
cargo test
```

## Future Enhancements

- Frontend implementation with interactive charts
- Support for additional cryptocurrency data sources
- User authentication for personalized news feeds
- WebSocket support for real-time updates
- Docker containerization
