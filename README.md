# How to start:
## Install [rust](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Set the correct addresses
In the `.env` file set the `SERVER` and `CLIENT` addresses to where you are hosting the server and client. E.g
```
SERVER=localhost:5000
CLIENT=localhost:5001
```

## run the client:
```bash
cargo run c
```
## run the server:
```bash
cargo run s
```
