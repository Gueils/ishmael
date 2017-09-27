FROM rust

RUN apt-get update && apt-get install -y libssl1.0-dev

RUN mkdir -p /code

ADD . /code

WORKDIR /code

RUN cargo install
