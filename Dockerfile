FROM rust as build

WORKDIR /workspace
COPY ./ /workspace

RUN cargo build --bin exit
RUN cargo build --bin helloworld
RUN cargo build --bin log
# end build

FROM kong:ubuntu

USER root
RUN apt-get update && apt-get install -y libprotobuf-dev

USER kong
COPY --from=build /workspace/target/debug/exit /usr/local/bin/exit-plugin
COPY --from=build /workspace/target/debug/helloworld /usr/local/bin/helloworld-plugin
COPY --from=build /workspace/target/debug/log /usr/local/bin/log-plugin
