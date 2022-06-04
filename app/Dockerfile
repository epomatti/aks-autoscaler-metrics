FROM rust

WORKDIR /usr/src/myapp
COPY . .
COPY config/example.env .env

RUN cargo install --path .

CMD ["icecream"]