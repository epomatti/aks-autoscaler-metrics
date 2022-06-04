FROM rust

WORKDIR /usr/src/myapp
COPY . .

ENV BINDING_HOST=0.0.0.0
ENV BINDING_PORT=8080

RUN cargo install --path .

CMD ["icecream"]