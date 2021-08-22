FROM rust:alpine as build
RUN apk add --no-cache alpine-sdk
RUN mkdir /app
COPY . /app
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/app/target \
    cd /app && cargo build --release && \
    cp /app/target/release/isodd /


FROM alpine
COPY --from=build /isodd .
ENTRYPOINT [ "/isodd" ]
