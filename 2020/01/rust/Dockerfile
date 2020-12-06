FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/aoc

COPY . .

RUN rustc -D warnings -o aoc main.rs

CMD ["./aoc"]