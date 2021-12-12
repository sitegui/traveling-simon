FROM rust:1.56 AS engine

WORKDIR /app
COPY engine/Cargo.lock engine/Cargo.toml ./
COPY engine/src/main.rs ./src/
RUN cargo build --release || true
COPY engine/ ./
RUN cargo build --release

FROM node:16

WORKDIR /app
RUN mkdir -p data
ENV DEBUG=web
ENV NODE_ENV=production
ENV ENGINE_PATH=data/traveling-simon

# Add Tini
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]

COPY --from=engine /app/target/release/traveling-simon data/

COPY web/package.json web/package-lock.json ./
RUN npm ci

COPY web/ ./

EXPOSE 3000
CMD ["node", "app.js"]
