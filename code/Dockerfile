FROM rustlang/rust:nightly as builder
ADD . /spargel
WORKDIR /spargel
RUN cargo build --release --locked && cp ./target/release/spargel /bin/spargel

# TODO: TEMPLATE BUILDING HERE

FROM node:current-alpine3.18 as sass_builder
ADD templates /spargel/templates
RUN mkdir /spargel/static && \ 
    apk add --no-cache git && \
    npm install -g sass  && \
    cd /spargel/templates/plain/sass && yarn install && sass main.scss /spargel/static/plain.min.css --style=compressed && yarn assets && \
    cd /spargel/templates/terminal.css/sass && yarn install && sass main.scss /spargel/static/terminal.min.css --style=compressed && yarn assets && \
    ls -lah /spargel/static

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
COPY --from=sass_builder /spargel/static /spargel/static
ADD Rocket.toml /spargel/Rocket.toml
EXPOSE 8000
WORKDIR /spargel
CMD ["/bin/spargel"]