# roguelike-rust

```bash
sudo apt-get update
sudo apt-get install -y curl file gcc g++ git make openssh-client \
    autoconf automake cmake libtool libcurl4-openssl-dev libssl-dev \
    libelf-dev libdw-dev binutils-dev zlib1g-dev libiberty-dev wget \
    xz-utils pkg-config python
sudo apt-get install libsdl2-dev
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
rustup update
rustup install nightly
rustup default nightly
rustup run nightly cargo build -p roguelike
```
