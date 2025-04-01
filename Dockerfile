FROM rust:slim-bullseye


WORKDIR /app

COPY * .

ADD https://www.transitchicago.com/downloads/sch_data/google_transit.zip /data/google_transit.zip

VOLUME ["/data"]

RUN cargo install --path .
