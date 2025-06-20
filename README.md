# portscanx
<p align="center">
    <img src="https://imgur.com/ZgMegPb.png" alt="portscanx logo">
</p>

> A blazing fast and developer-friendly network port scanner written in Rust.

---

## 🚀 Features

* ⚡ Fast scanning using async runtime
* 🧠 Developer-oriented output
* 📦 JSON export support
* 🔍 Supports TCP and UDP scanning (UDP support in progress)

---

## 📦 Installation

```bash
git clone https://github.com/shvvkz/portscanx.git
cd portscanx
make install
cd ..
rm -rf portscanx
```

---

## ⚙️ Usage

Basic scan:

```bash
portscanx 192.168.1.1
```

Scan a custom port range:

```bash
portscanx 192.168.1.1 -p 20-1000
```

Export results to JSON:

```bash
portscanx 192.168.1.1 -o json
```
*This will save the scan at portscanx.json*

View help:

```bash
portscanx --help
```

---

## 🛠️ Roadmap

* [x] Basic port scanning (TCP)
* [ ] Configurable parallelism (via CLI flag)
* [ ] Add an update command to allow updating portscanx using the `-u` flag
* [ ] Write documentation comments for functions and structures in the code
* [ ] UDP support
* [ ] Config file support

---

## 👨‍💻 Development

### Requirements

* Rust (>= 1.75)
* Cargo

### Build

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```

---

## 🤝 Contributing

Contributions, issues and feature requests are welcome! Feel free to open a pull request.

---

## 🔗 Links

* [GitHub Repository](https://github.com/shvvkz/portscanx)
* [Rust Language](https://www.rust-lang.org/)

---

## 🙌 Author

Made with ❤️ by [shvvkz](https://github.com/shvvkz)
