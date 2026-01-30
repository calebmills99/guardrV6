# DigitalOcean Deployment Tools Container
# Cross-platform container for running deployment scripts on Windows, macOS, and Linux

FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    ca-certificates \
    bash \
    openssl \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install doctl (DigitalOcean CLI)
ARG DOCTL_VERSION=1.104.0
RUN wget -q https://github.com/digitalocean/doctl/releases/download/v${DOCTL_VERSION}/doctl-${DOCTL_VERSION}-linux-amd64.tar.gz \
    && tar xf doctl-${DOCTL_VERSION}-linux-amd64.tar.gz \
    && mv doctl /usr/local/bin/ \
    && rm doctl-${DOCTL_VERSION}-linux-amd64.tar.gz \
    && chmod +x /usr/local/bin/doctl

# Create workspace directory
WORKDIR /workspace

# Copy scripts into container
COPY scripts/ /workspace/scripts/
RUN chmod +x /workspace/scripts/*.sh

# Copy app spec
COPY ..//.do/app.yaml /workspace/.do/app.yaml 2>/dev/null || true

# Set up entrypoint
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Set environment variables
ENV PATH="/workspace/scripts:${PATH}"

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["/bin/bash"]
