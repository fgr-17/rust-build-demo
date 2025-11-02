FROM rust:alpine3.22

ENV PROJECT_NAME=${PROJECT_NAME}

ENV PROJDIR=/workspace/${PROJECT_NAME}
ENV SCRIPTS_DIR=./scripts

# to run mockall
RUN apk add --no-cache musl-dev
RUN apk add make protoc flatc

RUN rustup update && rustup component add clippy rustfmt llvm-tools
RUN cargo install cargo-llvm-cov --locked

# Add ll alias for colored ls -la
RUN echo "alias ll='ls -la --color=auto'" > /etc/ash_aliases

# Set ENV so ash shell sources aliases for all interactive shells
ENV ENV=/etc/ash_aliases

COPY ${SCRIPTS_DIR}/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/sh"]


