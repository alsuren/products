# Setup rust build environment
FROM rust:1.41.0 AS build-context

RUN rustup component add rustfmt

WORKDIR /usr/src/doc-index-updater

COPY Cargo.toml .
COPY Cargo.lock .

# Layer hack: Build an empty program to compile dependencies and place on their own layer.
# This cuts down build time

# it was borrowed from here: 
# https://github.com/deislabs/krustlet/blob/master/Dockerfile#L7 
RUN mkdir -p ./src/ && \
  echo 'fn main() {}' > ./src/main.rs && \
  echo '' > ./src/lib.rs

RUN cargo fetch

RUN cargo build --release && \
  rm -rf ./target/release/.fingerprint/doc_index_updater-*

# Setup debian release environment
FROM debian:buster-slim AS release-context

RUN apt-get update && apt-get install -y \
  tini \
  ;

RUN useradd svc

ENV PORT=8000

# Build real binaries now, as late as possible
FROM build-context AS build

COPY ./src ./src

RUN cargo build --release

# Create the release
FROM release-context AS release

COPY --from=build /usr/src/doc-index-updater/target/release/doc_index_updater /

RUN chown -R svc /doc_index_updater

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/doc_index_updater"]