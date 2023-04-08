FROM rust:alpine as builder
RUN apk add --no-cache musl-dev upx clang mold

ARG project=backend

WORKDIR /build
COPY . .
RUN RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold" cargo build --release -p "$project" && \
    upx --best --lzma ./target/release/"$project"

FROM alpine

WORKDIR /app

RUN addgroup -S app && adduser -S app -G app

ARG project=backend
COPY --from=builder /build/target/release/"$project" /app/app
RUN chown app:app /app -R
USER app:app

ENTRYPOINT ["/app/app"]
