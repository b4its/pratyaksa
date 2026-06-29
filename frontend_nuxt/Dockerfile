# ---- Stage 1: Build ----
# Gunakan debian slim (glibc) agar native bindings kompatibel
FROM node:22-slim AS builder

WORKDIR /app

ARG NUXT_PUBLIC_API_BASE=/api/v1
ENV NUXT_PUBLIC_API_BASE=${NUXT_PUBLIC_API_BASE}
ENV CI=true

# Copy package files
COPY package.json package-lock.json* ./

# Install dependencies — tanpa legacy-peer-deps agar hoisting standar
RUN npm install

# Copy source
COPY app ./app
COPY server ./server
COPY nuxt.config.ts tsconfig.json ./
COPY public ./public

# Prepare types lalu build
RUN npx nuxt prepare
RUN npx nuxt build

# ---- Stage 2: Runtime ----
FROM node:22-slim AS runtime

WORKDIR /app

COPY --from=builder /app/.output ./.output

EXPOSE 3000

ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["node", ".output/server/index.mjs"]