FROM rust:alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/smolltech
COPY . .

RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/site /usr/local/bin/site
CMD ["site"]
