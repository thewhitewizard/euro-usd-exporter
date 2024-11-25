
My First prometheus exporter in Rust

## note to build for RPI 4

``` sh
sudo apt-get install -y gcc-aarch64-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
``` 

``` sh
curl localhost:8080/metrics

euro_usd_rate 0.9589901221
``` 

``` sh
euro-usd-exporter  | Starting server on port 8080...
euro-usd-exporter  | [2024-11-25 08:36:17] Refreshed exchange rate: euro_usd_rate = 0.9542101167
``` 
