# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.82.0
ARG APP_NAME=server
################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app
# Set environment variables.
ENV SQLX_OFFLINE=true


# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Copy the source code and Cargo files into the container.
COPY . .

# Build the application.
RUN cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application.

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 3001

# What the container should run when it is started.
CMD ["/bin/server"]