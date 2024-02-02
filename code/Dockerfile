FROM rustlang/rust:nightly as builder
ADD . /spargel
WORKDIR /spargel
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /spargel/target/spargel /spargel
CMD ["/spargel"]
