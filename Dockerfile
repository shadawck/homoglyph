FROM node:16.13.1 AS client
WORKDIR /app
COPY ./web-app/client .
RUN npm i
RUN npm run build

FROM rust:1.60 AS server
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./domain ./domain
COPY ./drivers ./drivers
COPY ./service ./service
COPY ./web-app/ ./web-app/src
COPY ./web-app/Cargo.toml ./web-app/Cargo.toml
COPY --from=client /app/build ./client/build
RUN cargo build --release
CMD ["./target/release/web-app"]