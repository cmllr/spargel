ADD . /spargel
WORKDIR /spargel
RUN cargo build --release --locked && cp ./target/release/spargel /bin/spargel

FROM debian:bullseye-slim AS final
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN mkdir /spargel
USER appuser

COPY --from=builder /bin/spargel /bin/
ADD templates /spargel/templates
ADD Rocket.toml /spargel/Rocket.toml
EXPOSE 8000
WORKDIR /spargel
CMD ["/bin/spargel"]