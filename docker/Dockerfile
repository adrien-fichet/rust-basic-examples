# docker build -t hello-docker . && docker image ls hello-docker && docker run --rm hello-docker

FROM rust:latest AS build
WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/docker /app/hello-docker

FROM alpine:latest
RUN apk add --no-cache gcompat libstdc++
RUN adduser --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
USER appuser
COPY --from=build /app/hello-docker /bin/hello-docker
CMD ["/bin/hello-docker"]
