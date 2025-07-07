# RustRecon

**RustRecon** is a simple and powerful passive reconnaissance CLI tool built with Rust. It allows users to gather DNS records, HTTP headers, and subdomains via `crt.sh`, providing a lightweight yet efficient information-gathering tool for security researchers and developers.

---

## 🚀 Features

* 🔍 Perform **DNS enumeration** (A, AAAA, MX, NS, TXT records)
* 🌐 Fetch **HTTP headers** from a given URL
* 🕵️ Discover **subdomains** using certificate transparency logs (via `crt.sh`)

---

## 🛠 Built With

* [Rust](https://www.rust-lang.org/)
* [Clap](https://crates.io/crates/clap) for argument parsing
* [Reqwest](https://crates.io/crates/reqwest) for HTTP requests
* [Trust-DNS](https://crates.io/crates/trust-dns-resolver) for DNS resolution
* [Serde](https://crates.io/crates/serde) for JSON parsing

---

## 📦 Installation

### Option 1: Build from source (Rust required)

```bash
# Clone the repository
$ git clone https://github.com/Selvaganapathy2004/rust_recon.git
$ cd rust_recon

# Build release binary
$ cargo build --release

# Run
$ ./target/release/rust_recon --help
```

### Option 2: Download Prebuilt Binary (No Rust Needed)

Visit the [Releases](https://github.com/Selvaganapathy2004/rust_recon/releases) page and download the latest `rust_recon.exe`.

```bash
# After downloading:
$ ./rust_recon.exe --help
```

---

## 📋 Usage

```bash
rust_recon.exe <COMMAND> [OPTIONS]
```

### Commands

* `dns -t <target>` : Perform DNS lookup for the target domain
* `headers -u <url>` : Fetch HTTP headers from the given URL
* `subdomains -d <domain>` : Discover subdomains via crt.sh

### Examples

```bash
rust_recon.exe dns -t example.com
rust_recon.exe headers -u https://example.com
rust_recon.exe subdomains -d example.com
```

---

## 📂 Project Structure

```
rust_recon/
├── src/
│   └── main.rs        # Main CLI logic and features
├── Cargo.toml         # Project configuration
└── README.md          # Project documentation
```

---

## 🤝 Contributing

Pull requests are welcome! Feel free to fork this repo and submit improvements or fixes.

---

## 📄 License

This project is licensed under the MIT License.

---

## 🙋‍♂️ Author

**Selvaganapathy**
GitHub: [@Selvaganapathy2004](https://github.com/Selvaganapathy2004)
