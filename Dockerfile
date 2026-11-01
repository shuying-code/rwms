# ---- build stage ----
FROM rust:1-bookworm AS build
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN cargo build --release

# ---- runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/rwms /app/rwms
COPY --from=build /app/static /app/static
COPY --from=build /app/application.yml /app/application.yml
RUN mkdir -p /wms
ENV PORT=8080
EXPOSE 8080
VOLUME ["/wms"]
CMD ["/app/rwms"]
