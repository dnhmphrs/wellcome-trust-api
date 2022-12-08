# wellcome-trust-api
API for Wellcome Trust Code Challenge

##  /python - Python Build
- Rebuild in Python after Rust hurt my soul
- Run Local Server - available at localhost:5000
- Current Date is set to 2021-12-31 to make the code run better, API data is out of date.
- No Testing implented, and .env values stored in main file.

###  Run Local Server
```
$ cd /python
$ flask run
```

##  /rust - Incomplete Rust Build
- Derived from: https://github.com/actix/examples/tree/master/https-tls/awc-https
- Cargo is required to compile locally: https://www.rust-lang.org/tools/install
- Server/Client works, but data structure manipulation in rust was too hard so switched to python to complete challenge.

###  Cargo Commands

```
$ cd /rust
$ cargo build
$ cargo run
$ cargo clean
```
