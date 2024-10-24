# We use the latest Rust stable release as base image
FROM rust:1.81.0

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration.
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
# Copy all files from our working environment to our Docker image.
RUN cargo build --release

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
