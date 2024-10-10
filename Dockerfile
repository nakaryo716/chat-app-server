FROM rust:latest

WORKDIR /app
COPY . /app
RUN cargo install sqlx-cli
CMD [ "make" ]
