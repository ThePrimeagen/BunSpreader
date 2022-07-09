FROM node:lts-alpine

WORKDIR /app
COPY package.fastify.json /app/package.json
RUN npm install
COPY src/fastify.js /app/fastify.js

CMD ["node", "/app/fastify.js"]


