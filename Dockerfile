FROM rust as build

WORKDIR /workspace

RUN rustup component add rustfmt

COPY ./ /workspace

RUN cargo build --bin helloworld
# end build

FROM kong:ubuntu

USER root
RUN apt-get update && apt-get install -y libprotobuf-dev

USER kong
COPY --from=build /workspace/target/debug/helloworld /usr/local/bin/example-rust-plugin
