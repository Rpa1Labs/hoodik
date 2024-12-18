FROM ubuntu:latest AS hookik-back

ENV HOST 0.0.0.0
EXPOSE 5443

RUN apt update && apt install curl wget libpq-dev clang llvm pkg-config nettle-dev libc6-dev libssl-dev -y

#install rust
RUN cd /tmp/ && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . "$HOME/.cargo/env" 

ENV DATA_DIR="/data"
ENV RUST_LOG="hoodik=debug,auth=debug,error=debug,entity=debug,storage=debug,context=debug,util=debug,cryptfns=debug,actix_web=debug"

# build and run backend
WORKDIR /hoodik
CMD . "$HOME/.cargo/env" && cargo build -F dev-front --bin hoodik --release && ./target/release/hoodik -a 0.0.0.0 -p 5443
