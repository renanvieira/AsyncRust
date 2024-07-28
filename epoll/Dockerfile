FROM rust:1-alpine

WORKDIR /usr/a-epoll

COPY . .

RUN cargo build

CMD ["cargo", "run", "--", "delayserver"]
