FROM rust:1.94-slim-bullseye AS builder
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV GIT_SSL_NO_VERIFY=true

WORKDIR /usr/app
COPY . /usr/app/
RUN cargo build --release


FROM rust:1.94-slim-bullseye AS runner
WORKDIR /usr/app
COPY --from=builder target/ /usr/app/
EXPOSE 3000
CMD ["cargo", "run"]