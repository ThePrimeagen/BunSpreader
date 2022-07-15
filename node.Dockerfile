FROM node:lts-alpine

WORKDIR /app
COPY package.fastify.json /app/package.json
RUN npm install
RUN npx tsc
COPY src/fastify.js /app/fastify.js
COPY src/boofer.

CMD ["node", "/app/fastify.js"]


