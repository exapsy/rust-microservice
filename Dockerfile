FROM rustlang/rust:nightly

WORKDIR /usr/src/rust-microservice
COPY . .

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo install --path .

CMD ["service"]
