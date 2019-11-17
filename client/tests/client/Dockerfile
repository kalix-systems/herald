FROM  rabits/qt:5.13-desktop

RUN sudo apt-get update
RUN sudo apt-get install -y \
	libdbus-1-dev

RUN curl -sSf https://sh.rustup.rs | sh  -s -- -y
RUN git clone https://github.com/kalix-systems/herald && cd herald/libherald && $HOME/.cargo/bin/cargo build


