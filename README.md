# rust-learning

这是一个 **Rust 异步编程学习项目**，用于练习 Rust 中的异步运行时、异步 I/O、TCP 监听以及简单的 HTTP 响应处理。

项目目前基于 [`tokio`](https://tokio.rs/) 实现了一个简易 Web 服务，可以监听本地端口并根据请求路径返回不同的 HTML 页面。

## 学习目标

- 理解 Rust 中 `async` / `await` 的基本用法
- 学习使用 `tokio` 异步运行时
- 练习异步 TCP 服务端开发
- 使用异步文件读取返回静态 HTML 内容
- 了解 HTTP 请求行、状态码和响应体的基础结构

## 项目结构

```text
.
├── src/
│   └── main.rs      # 异步 TCP 服务端入口
├── index.html       # 首页
├── sleep.html       # 延迟响应页面
├── 404.html         # 404 页面
├── Cargo.toml       # Rust 项目配置和依赖
└── Cargo.lock
```

## 主要功能

- 监听本地地址 `127.0.0.1:7878`
- 访问 `/` 返回 `index.html`
- 访问 `/sleep` 等待 2 秒后返回 `sleep.html`
- 访问其他路径返回 `404.html`
- 使用 `tokio::fs` 异步读取 HTML 文件
- 使用 `tokio::net::TcpListener` 和 `TcpStream` 处理网络连接

## 运行项目

确保已经安装 Rust 工具链，然后在项目根目录运行：

```bash
cargo run
```

启动后，在浏览器中访问：

```text
http://127.0.0.1:7878
```

也可以访问延迟响应示例：

```text
http://127.0.0.1:7878/sleep
```

## 依赖

当前主要依赖：

- `tokio`：异步运行时和异步 I/O 支持
- `anyhow`：简化错误处理
