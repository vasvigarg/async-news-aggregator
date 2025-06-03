# 📰 Async News Aggregator

An asynchronous Rust CLI application that fetches the top 5 latest news headlines from multiple Indian news websites using `reqwest`, `tokio`, and `scraper`.

## 🚀 Features

- Scrapes latest headlines from:
  - The Hindu 🕉️
  - Livemint 💸
- Fetches all in parallel using async/await
- Simple terminal-based output
- Handles HTML structure dynamically with CSS selectors

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/)
- [Tokio](https://crates.io/crates/tokio)
- [Reqwest](https://crates.io/crates/reqwest)
- [Scraper](https://crates.io/crates/scraper)
- [Futures](https://crates.io/crates/futures)

## 📦 Installation

Clone the repo and build it:

```bash
git clone https://github.com/your-username/async-news-aggregator.git
cd async-news-aggregator
cargo build --release
```

## 🧪 Running the App

```bash
cargo run
```

Sample output:

```bash
🔄 Fetching latest news...

🖋️ LATEST NEWS HEADLINES FROM THE HINDU:

📰 Headline 1
🔗 https://www.thehindu.com/article-link-1

📰 Headline 2
🔗 https://www.thehindu.com/article-link-2
...

💸 LATEST NEWS HEADLINES FROM LIVEMINT:
...

🇮🇳 LATEST NEWS HEADLINES FROM INDIAN EXPRESS:
...

🎉 That’s your quick news round-up!
```

## 🧩 Notes

- Make sure you're connected to the internet.

- If the selectors change on news websites, you may need to update them in main.rs.

---

Feel free to contribute or open issues for any bugs or feature requests!
