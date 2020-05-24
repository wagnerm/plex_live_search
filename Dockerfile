FROM rust:1.43.1

RUN mkdir -p /plex_live_search /var/tmp/
WORKDIR /plex_live_search

USER root
ENV USER root
RUN cargo init

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . ./

RUN cargo build --release && \
    mv target/release/plex_live_search /usr/local/bin/

ENTRYPOINT /usr/bin/bin/plex_live_search
