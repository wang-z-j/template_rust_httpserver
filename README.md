## 项目版本

- **`feature/http_server1.0`**：使用 `TcpListener` 实现基本的 HTTP 服务器，支持接收并响应 HTTP 请求。
- **`feature/http_server2.0`**：在 `feature/http_server1.0` 的基础上实现了基本的路由系统，能够根据不同的 URL 路径响应不同的内容。
- **`master`**：使用 `Actix Web` 框架实现了完整的 HTTP 服务，支持异步请求处理、路由匹配、JSON 格式响应等功能。

## 项目目的

- 构建一个从基础到复杂的 HTTP 服务器，帮助理解 HTTP 服务的实现过程。
- 提供一个清晰的演示，从最基础的 TCP 服务器实现到使用现代 Web 框架进行开发。
- 展示如何扩展 Web 服务器的功能，包括路由、JSON 支持和错误处理。

## 功能概述

### `feature/1.0`：使用 `TcpListener` 实现基本 HTTP 服务器

- **TCP 监听**：使用标准库中的 `TcpListener` 监听客户端连接。
- **基础 HTTP 响应**：通过手动处理 HTTP 请求和响应，返回基本的 HTTP 响应。
- **无路由支持**：所有请求返回相同的响应，没有路由功能。

### `feature/2.0`：基本 HTTP 服务器与路由处理

- **HTTP 服务器**：通过 `TcpListener` 实现了基本的 HTTP 服务器。
- **路由支持**：实现了基本的路径路由，根据不同的请求路径提供不同的响应。
- **JSON 响应**：支持将数据结构转为 JSON 格式响应。
- **基本错误处理**：增加了基础的错误响应功能。

### `master`：基于 `Actix Web` 的完整 HTTP 服务

- **Actix Web 框架**：使用 `Actix Web` 框架来构建更强大的 HTTP 服务，支持路由和异步请求处理。
- **异步支持**：采用异步编程模型，能够高效处理并发请求。
- **路由系统**：根据 HTTP 请求的路径和方法来处理不同的路由。
- **JSON 响应支持**：通过 `serde` 支持 JSON 数据的自动序列化与反序列化。
- **增强的错误处理**：提供更详细的错误处理和定制的错误响应。

## 技术栈

- **Rust**：现代的系统编程语言，注重性能和内存安全。
- **Actix Web**：高效的 Web 框架，广泛用于构建 Web 服务。
- **Serde**：Rust 的序列化和反序列化库，支持 JSON 格式。
- **Tokio**：Rust 的异步运行时，支持高并发和异步 I/O。
- **TCP Listener**：用于监听网络连接，是实现最基础 HTTP 服务器的关键。

## 安装与运行

### 克隆项目

```bash
git clone https://github.com/wang-z-j/template_rust_httpserver.git
cd template_rust_httpserver
```

### 安装依赖并运行

```bash
cargo run
```

### 访问服务

服务器启动后，可以通过浏览器或工具访问以下端点：

- `GET http://127.0.0.1:7878/` - 返回一个欢迎页面。
- `GET http://127.0.0.1:7878/user` - 返回一个包含用户信息的 JSON 响应。
