FROM node:lts-alpine

WORKDIR /app
COPY package.fastify.json /app/package.json
run npm i
COPY src/fastify.js /app/fastify.js
COPY dist/src/boofer.js /app/boofer.js

CMD ["node", "/app/fastify.js"]


