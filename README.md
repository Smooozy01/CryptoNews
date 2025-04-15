# 📰 Crypto News Service

A simple Rust web service that lets users search for the latest cryptocurrency-related news using a basic HTML interface. This app queries real-time news data from external APIs and displays results in a readable format.

## 🔧 Features

- Search news by cryptocurrency name or symbol (e.g., `BTC`, `Ethereum`)
- Fetches data from external APIs like [NewsData.io](https://newsdata.io/)
- Displays:
  - Title (linked to full article)
  - Source
  - Published Date
  - Summary
- Clean, minimal web UI

## 🚀 Tech Stack

- **Backend:** [Axum](https://docs.rs/axum), [Reqwest](https://docs.rs/reqwest)
- **Frontend:** HTML/CSS (served from Rust)
- **Async runtime:** Tokio
- **News API:** [NewsData.io](https://newsdata.io/)

## 🧪 Screenshots

![screenshot](docs/screenshot.png) <!-- Optional -->

## 🛠️ Getting Started

### Prerequisites

- Rust installed: [https://rustup.rs](https://rustup.rs)

### 1. Clone the Repo

```bash
git clone https://github.com/yourusername/crypto-news-service
cd crypto-news-service
```

### 2. Get Your NewsData.io API Key

- Go to https://newsdata.io
- Sign up for a free account
- Copy your API key (e.g., pub_XXXXXXXXXX)

### 3. Add the API Key

- Open main.rs and replace this line with your API-KEY:
```Rust
let api_key = "pub_REPLACE_WITH_YOUR_KEY";
```

### 4. Run the Project

```bash
cargo run
```
- Open your browser at: http://localhost:3000

## Output
<img width="291" alt="{942CAE16-23CE-437B-AEF8-672A410F6C24}" src="https://github.com/user-attachments/assets/474d874f-788d-4835-b6d8-6797b477203d" />
<img width="867" alt="{B20D2D13-9552-43DC-B522-F4215B7CF4DE}" src="https://github.com/user-attachments/assets/adbf29c8-9125-411d-bc10-ae42d1663b0c" />

## 📦 Dependencies
    axum

    reqwest

    tokio

    serde, serde_json

    chrono

See Cargo.toml for full list.

## ⚠️ Notes

    This demo uses NewsData.io free tier; there may be rate limits.

    Error handling is built in — if the API fails, it will show fallback messages.
