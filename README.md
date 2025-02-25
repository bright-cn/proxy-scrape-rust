[![Promo](https://github.com/bright-cn/Google-News-Scraper/blob/main/Proxies%20and%20scrapers%20GitHub%20bonus%20banner%20CN.png?md5=105367-daeb786e)](https://www.bright.cn/?promo=github15)
# 代理爬虫 Rust

一个用于演示如何通过代理服务器进行网页爬取的 Rust 基本 API 程序

本项目演示了在 [Rust 中进行网页爬取](https://www.bright.cn/blog/how-tos/web-scraping-with-rust) 时如何设置代理服务器。代理服务器在网页爬取过程中通过使用它们的 IP 地址来保护您的数字身份，从而绕过 IP 封禁和地理位置封锁。

本仓库包含了基于 [Rust Proxy Servers](https://www.bright.cn/blog/how-tos/rust-proxy-servers) 文章的不同示例程序：
- basic：展示了获取数据并解析的基本用法
- basic_proxy：展示了如何使用基本代理来爬取数据（需要配置 nginx 服务器并使用 [nginx.conf](nginx.conf) 文件更新其配置）
- rotating_proxy：展示了如何使用轮换代理来爬取数据（需要配置 nginx 服务器并使用 [nginx.conf](nginx.conf) 文件更新其配置）
- brightdata_proxy：展示了如何使用 Bright Data 代理来爬取数据（需要 [Bright Data 代理配置](#bright-data-proxy-configuration)）

## 前置条件
- Rust 和 Cargo (安装指南: [Rustup](https://rustup.rs/))  
- Nginx (安装指南: [nginx](https://nginx.org/en/docs/install.html))

## 设置
1. 克隆本仓库: `git clone git@github.com:luminati-io/proxy-scrape-rust.git`
2. 进入项目目录: `cd proxy-scrape-rust`
3. 安装依赖: `cargo build`

## 运行爬虫
使用 Cargo 运行爬虫：
```bash
cargo run
```
或者执行以下其中一个命令：
```bash
cargo run --bin (basic|basic_proxy|rotating_proxy|brightdata_proxy)
```

## 依赖
本项目使用了多个外部库来实现关键功能。以下是主要依赖：

### 1. Scraper
[Scraper](https://crates.io/crates/scraper) 是一个基于 CSS 选择器解析 HTML 文档的 Rust crate。它使用 `html5ever` 库来遵从 HTML5 规范，提供健壮且高效的解析能力。该库非常适合从 HTML 内容中提取数据，是网页爬取任务中的理想选择。

### 2. Reqwest
[Reqwest](https://crates.io/crates/reqwest) 是一个符合人体工学、功能完善的 Rust HTTP 客户端。它同时支持同步和异步请求，并提供 JSON 和流式响应等功能。该库使网络请求变得简单高效，能够轻松处理各种 HTTP 相关任务。

### 3. Tokio
[Tokio](https://crates.io/crates/tokio) 是一个事件驱动、非阻塞 I/O 平台，可用于在 Rust 中编写异步应用程序。它基于 Rust 的 async/await 特性，使编写可扩展、高性能的应用程序变得更加简单。Tokio 在处理网络服务中的并发操作和异步任务时至关重要。

如需了解更多关于这些依赖的使用示例和文档，请参阅它们各自的文档。

## Bright Data 代理配置
要运行本项目，请确保您拥有可用的代理服务器。您可以从 [Bright Data](https://www.bright.cn/) 等提供商获取代理服务器信息。获取到代理服务器的详细信息后，在 `main.rs` 文件中使用相应的配置信息进行更新。

## 贡献
欢迎贡献！如果您发现任何问题或有改进建议，欢迎提交 issue 或发起 pull request。

## 许可证
本项目基于 MIT 许可证授权。更多信息请参阅 [LICENSE](LICENSE) 文件。
