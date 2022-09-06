FROM node:16.13.1 AS client
WORKDIR /app
COPY ./homoglyph-client .
RUN npm i
RUN npm run build

FROM rust:1.60 AS server
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./homoglyph-domain ./homoglyph-domain
COPY ./homoglyph-driver ./homoglyph-driver
COPY ./homoglyph-service ./homoglyph-service
COPY ./homoglyph-server/ ./homoglyph-server/src
COPY ./web-app/Cargo.toml ./web-app/Cargo.toml
COPY --from=client /app/build ./client/build
RUN cargo build --release
CMD ["./target/release/homoglyph-server"]