# Multi-stage build for ParaForge

# Stage 1: Build Rust CLI
FROM rust:1.75 as rust-builder

WORKDIR /build

# Copy workspace files
COPY Cargo.toml ./
COPY cli ./cli
COPY core ./core
COPY simulator ./simulator

# Build release binary
RUN cargo build --release --bin paraforge

# Stage 2: Build Node.js SDK and Frontend
FROM node:20-alpine as node-builder

WORKDIR /build

# Build SDK
COPY sdk/package.json sdk/package-lock.json ./sdk/
WORKDIR /build/sdk
RUN npm ci
COPY sdk/src ./src
COPY sdk/tsconfig.json ./
RUN npm run build

# Build Frontend
WORKDIR /build/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend ./
RUN npm run build

# Stage 3: Final runtime image
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust binary
COPY --from=rust-builder /build/target/release/paraforge /usr/local/bin/paraforge

# Copy SDK build
COPY --from=node-builder /build/sdk/dist /opt/paraforge/sdk

# Copy Frontend build
COPY --from=node-builder /build/frontend/.next /opt/paraforge/frontend/.next
COPY --from=node-builder /build/frontend/public /opt/paraforge/frontend/public
COPY --from=node-builder /build/frontend/package.json /opt/paraforge/frontend/

# Install Node.js for frontend
COPY --from=node:20-alpine /usr/local/bin/node /usr/local/bin/
COPY --from=node:20-alpine /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm

# Set up working directory
WORKDIR /app

# Expose ports
EXPOSE 8545 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:8545/ || exit 1

# Default command
CMD ["paraforge", "node", "--port", "8545"]
