# 🎬 NetMirror Backend

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.70+-CE422B?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/Status-Active-green?style=for-the-badge)](https://github.com/harshitsharmaaaa/Net_Mirror_backend)

A high-performance Netflix clone backend built with **Rust** and **Axum** web framework.

[Features](#-features) • [Installation](#-installation) • [Getting Started](#-getting-started) • [Project Structure](#-project-structure)

</div>

---

## 📚 Overview

NetMirror is a robust, fast, and scalable backend service for a Netflix-like streaming platform. Built with modern Rust technologies, it leverages the power of async/await, type safety, and zero-cost abstractions to deliver an efficient streaming service backend.

Whether you're looking to stream movies, manage user accounts, or build a media platform, NetMirror provides the foundation you need.

---

## ✨ Features

- ⚡ **High-Performance** - Built with Rust for speed and reliability
- 🔄 **Async/Await** - Full async support with Tokio runtime
- 🌐 **RESTful API** - Clean and intuitive API endpoints powered by Axum
- 🔐 **CORS Support** - Pre-configured cross-origin request handling
- 📦 **Type-Safe** - Leverages Rust's strong type system to prevent runtime errors
- 🚀 **Scalable** - Designed for horizontal scalability and high throughput
- 📝 **JSON Support** - Native JSON serialization with Serde
- 🔌 **HTTP Client** - Built-in HTTP client for external integrations

---

## 🛠️ Tech Stack

| Technology | Version | Purpose |
|-----------|---------|---------|
| **Rust** | 2024 Edition | Core Language |
| **Axum** | 0.8.9 | Web Framework |
| **Tokio** | 1.29.1 | Async Runtime |
| **Serde** | 1.0.188 | Serialization/Deserialization |
| **Tower-HTTP** | 0.5 | HTTP Utilities (CORS, Static Files) |
| **Reqwest** | 0.11.18 | HTTP Client |
| **Dotenv** | 0.15.0 | Environment Management |

---

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70 or later ([Install Rust](https://rustup.rs/))
- **Cargo** (comes with Rust)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/harshitsharmaaaa/Net_Mirror_backend.git
   cd Net_Mirror_backend
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Install dependencies and build**
   ```bash
   cargo build
   ```

4. **Run the development server**
   ```bash
   cargo run
   ```

   The server will start on the default port (typically `http://localhost:3000`)

### Running in Production

```bash
cargo build --release
./target/release/NetMirror
```

---

## 📁 Project Structure

```
Net_Mirror_backend/
├── src/                      # Source code directory
│   └── main.rs              # Application entry point
├── Cargo.toml               # Project configuration & dependencies
├── Cargo.lock               # Locked dependency versions
├── .env.example             # Example environment variables
├── .gitignore               # Git ignore rules
└── README.md                # This file
```

---

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
# Server Configuration
PORT=3000
HOST=0.0.0.0

# Database (if applicable)
DATABASE_URL=your_database_url

# Other settings
ENVIRONMENT=development
```

### Build Configuration

Edit `Cargo.toml` to customize:
- Project name and version
- Dependencies and their versions
- Build profiles (debug/release)

---

## 📡 API Usage

### Basic Example

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Welcome to NetMirror!"
}
```

---

## 📦 Dependencies Overview

| Package | Version | Features |
|---------|---------|----------|
| **axum** | 0.8.9 | Web framework with routing |
| **tokio** | 1.29.1 | Async runtime with full features |
| **serde** | 1.0.188 | Serialization with derive macros |
| **serde_json** | 1.0.145 | JSON processing |
| **tower-http** | 0.5 | CORS & static file serving |
| **reqwest** | 0.11.18 | HTTP client with TLS support |
| **dotenv** | 0.15.0 | Environment variable loading |

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/AmazingFeature`)
3. **Commit your changes** (`git commit -m 'Add some AmazingFeature'`)
4. **Push to the branch** (`git push origin feature/AmazingFeature`)
5. **Open a Pull Request**

Please ensure your code follows Rust conventions and include tests where applicable.

---

## 📋 Development Workflow

### Running Tests

```bash
cargo test
```

### Checking Code Quality

```bash
cargo clippy
```

### Formatting Code

```bash
cargo fmt
```

### Building Documentation

```bash
cargo doc --open
```

---

## 🐛 Known Issues & Roadmap

- [ ] Database integration (PostgreSQL)
- [ ] Authentication & Authorization
- [ ] User profile management
- [ ] Movie/Show catalog system
- [ ] Watch history tracking
- [ ] Recommendation engine
- [ ] Deployment documentation

---

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

---

## 👨‍💻 Author

**Harshit Sharma**
- GitHub: [@harshitsharmaaaa](https://github.com/harshitsharmaaaa)

---

## 📞 Support

If you encounter any issues or have questions:

- 📌 Open an [issue](https://github.com/harshitsharmaaaa/Net_Mirror_backend/issues)
- 💬 Check [discussions](https://github.com/harshitsharmaaaa/Net_Mirror_backend/discussions)
- 📧 Contact the maintainers

---

## ⭐ Show Your Support

If you like this project, please consider giving it a ⭐ on GitHub!

---

<div align="center">

**Made with ❤️ by Harshit Sharma**

</div>