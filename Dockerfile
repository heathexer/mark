FROM balenalib/raspberry-pi-debian:buster

RUN apt-get -q update && apt-get install -yq --no-install-recommends \
    build-essential git \
    vim \
    python2.7-dev python-pillow python3-dev python3-pillow \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /src/
RUN git clone https://github.com/hzeller/rpi-rgb-led-matrix.git

WORKDIR /src/rpi-rgb-led-matrix
RUN make install-python HARDWARE_DESC=adafruit-hat PYTHON=$(which python3)
    
CMD bash