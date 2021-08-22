FROM rust:alpine as build
RUN apk add --no-cache alpine-sdk
RUN mkdir /app
COPY . /app
RUN --mount=type=cache,target=/root/.cargo/registry \
    cd /app && cargo build --release


FROM alpine
COPY --from=build /app/target/release/isodd .
ENTRYPOINT [ "/isodd" ]
