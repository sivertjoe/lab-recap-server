# How to start:
## Install [rust](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
## Install these dependencies that you probably need:
```bash
sudo apt install build-essential
sudo apt install libssl-dev
sudo apt install pkg-config
```
For OSs that don't use apt, just google `install {dependency} {my os}`
## Set the correct addresses
In the `.env` file set the `SERVER` and `CLIENT` addresses to where you are hosting
the server and client. E.g
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
