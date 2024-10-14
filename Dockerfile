# We use the latest Rust stable release as base image
FROM lukemathwalker/cargo-chef:latest-rust-1.81.0 as chef
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration.
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
# Build our project dependacies, not our application!
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this our dependacy tree stays the same.
COPY . .
ENV SQLX_OFFLINE true
# Copy all files from our working environment to our Docker image.
RUN cargo build --release --bin zero2prod
# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL, ca-certificates
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment.
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY src/configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]
