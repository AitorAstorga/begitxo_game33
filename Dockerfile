# ----------------------------------------------
# Install Dependencies and Build
# ----------------------------------------------
FROM rust:1-slim as builder

# Install system dependencies.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
        curl \
        tar && \
    rm -rf /var/lib/apt/lists/*

# Add the wasm target 
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy the project files.
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application with Trunk (the generated files will be included in the build).
RUN cargo build --target wasm32-unknown-unknown --release

# ----------------------------------------------
# Launch with Nginx and the Image List Generator
# ----------------------------------------------
FROM nginx:bookworm

# Remove the default Nginx static assets.
RUN rm -rf /usr/share/nginx/html/*

# Copy the built assets from the builder stage.
COPY --from=builder /app/target/wasm32-unknown-unknown/release/begitxo_game33.wasm /usr/share/nginx/html
COPY img /usr/share/nginx/html/img
COPY sound /usr/share/nginx/html/sound
COPY index.html /usr/share/nginx/html

# Set the working directory.
WORKDIR /usr/share/nginx/html

EXPOSE 80

# Start the image list generator in the background and launch Nginx.
CMD sh -c "nginx -g 'daemon off;'"