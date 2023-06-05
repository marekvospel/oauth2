FROM node:18-alpine3.18 AS pruner
RUN apk add --no-cache libc6-compat
RUN corepack enable pnpm && npm i -g turbo

WORKDIR /build
COPY . .

RUN turbo prune --scope="@vospel/oauth2-frontend" --docker

FROM node:18-alpine3.18 AS builder
RUN apk add --no-cache libc6-compat
RUN corepack enable pnpm && npm i -g turbo

WORKDIR /build
COPY .gitignore .gitignore
COPY --from=pruner /build/out/json/ .
COPY --from=pruner /build/out/pnpm-lock.yaml .
RUN pnpm i

COPY --from=pruner /build/out/full/ .
RUN turbo run build --filter=@vospel/oauth2-frontend

FROM node:18-alpine3.18
RUN corepack enable pnpm && npm i -g turbo

WORKDIR /app
COPY --from=builder /build/ ./
RUN pnpm i

EXPOSE 80/tcp

ENV PORT=80

ENTRYPOINT ["node", "packages/frontend/.output/server/index.mjs"]