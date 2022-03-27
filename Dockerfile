FROM balenalib/raspberry-pi-debian:buster

# Installations
RUN apt-get -q update && apt-get install -yq --no-install-recommends \
    build-essential git \
    vim curl \
    python3-dev python3-pillow python3-numpy \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Get Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    --default-host armv7-unknown-linux-gnueabihf \
    --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

# Set timezone
RUN rm /etc/localtime && \
ln -s /usr/share/zoneinfo/America/Los_Angeles /etc/localtime

# Get rpi-rgb-matrix library
# WORKDIR /src
# RUN git clone https://github.com/hzeller/rpi-rgb-led-matrix.git

# WORKDIR /src/rpi-rgb-led-matrix
# RUN make install-python HARDWARE_DESC=adafruit-hat PYTHON=$(which python3)

COPY src /src/app
WORKDIR /src/app/rs

# Build Rust app
RUN cargo build --release || true

    
CMD cargo run --release || bash
