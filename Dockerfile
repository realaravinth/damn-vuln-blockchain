FROM rust:latest as build

COPY . /src
RUN cargo install --path /src

FROM debian:buster

LABEL org.opencontainers.image.source https://github.com/realaravinth/damn-vuln-blockchain

RUN set -ex; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev; \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/local/cargo/bin/dwb /usr/local/bin

RUN useradd -ms /bin/bash -u 1001 dwb
WORKDIR /home/dwb
USER dwb

EXPOSE 8080
ENTRYPOINT [ "/usr/local/bin/dwb" ]
