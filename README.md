[![推广](https://www.bright.cn/static/github_promo_15.png?md5=105367-daeb786e)](https://www.bright.cn/?promo=github15) 

# proxy-scrape-rust
基础 Rust API 程序，演示如何通过代理服务器进行网页爬虫

本项目演示了如何在 [Rust 网页爬虫](https://www.bright.cn/blog/how-tos/web-scraping-with-rust)中设置代理服务器。代理通过使用其 IP 地址，在网页爬虫过程中保护你的数字身份，并绕过 IP 封禁和地理限制。

此代码仓库包含多种可运行的程序，详情请参阅 [Rust 代理服务器](https://www.bright.cn/blog/how-tos/rust-proxy-servers)一文：
- basic：展示获取和解析数据的基本用法
- basic_proxy：展示如何使用基础代理抓取数据（需要设置 nginx 服务器，并使用 [nginx.conf](nginx.conf) 文件更新其配置）
- rotating_proxy：展示如何使用动态代理抓取数据（需要设置 nginx 服务器，并使用 [nginx.conf](nginx.conf) 文件更新其配置）
- brightdata_proxy：展示如何使用 Bright Data 代理抓取数据（需要完成 [Bright Data 代理配置](#bright-data-代理配置)）

## 前提条件
- Rust 和 Cargo（安装指南：[Rustup](https://rustup.rs/)）
- Nginx（安装指南：[nginx](https://nginx.org/en/docs/install.html)）

## 设置
1. 克隆代码仓库：`git clone git@github.com:luminati-io/proxy-scrape-rust.git`
2. 进入项目目录：`cd proxy-scrape-rust`
3. 安装依赖项：`cargo build`

## 运行爬虫工具
使用 Cargo 运行爬虫：
```bash
cargo run
```
或运行以下命令之一 
```bash
cargo run --bin (basic|basic_proxy|rotating_proxy|brightdata_proxy)
```
## 依赖项
本项目使用多个外部库来确保高效运行。以下是主要依赖项：

### 1. Scraper
[Scraper](https://crates.io/crates/scraper)是一个 Rust crate，可根据 CSS 选择器解析 HTML 文档。它使用符合 HTML5 规范的 `html5ever` 库，确保提供稳健且高效的解析能力。该库对于从 HTML 内容中提取数据至关重要，非常适合网页爬虫任务。

### 2. Reqwest
[Reqwest](https://crates.io/crates/reqwest)是一个符合人体工程学设计、功能完备的 Rust HTTP 客户端。它支持同步和异步请求，并提供 JSON 和流式响应等功能。该库简化了网络请求流程，能够高效处理各种 HTTP 相关任务。

### 3. Tokio
[Tokio](https://crates.io/crates/tokio)是一个事件驱动、非阻塞的 I/O 平台，用于通过 Rust 编写异步应用程序。它基于 Rust 的 async/await 功能构建，让可扩展、高性能应用程序的开发更加简单。Tokio 对于管理异步任务和计时器至关重要，尤其适合处理网络服务中的并发操作。

有关如何使用这些依赖项的更多信息和示例，请参阅各自的文档。

## Bright Data 代理配置
要运行本项目，请确保你拥有有效的代理服务器。你可以从 [Bright Data](https://www.bright.cn/) 等提供商处获取代理服务器详细信息。获取代理服务器详细信息后，请在 `main.rs` 文件中更新相应的代理配置。

## 贡献
欢迎贡献！如果你发现任何问题或有改进建议，请提交 issue 或 pull request。

## 许可证
本项目采用 MIT 许可证。有关更多信息，请参阅 [LICENSE](LICENSE) 文件。
