FROM debian:stable-slim as get

WORKDIR /bun

RUN apt update && apt install curl unzip -y
RUN curl --fail --location --progress-bar --output "/bun/bun.zip" "https://github.com/Jarred-Sumner/bun/releases/download/bun-v0.1.1/bun-linux-x64.zip"
RUN unzip -d /bun -q -o "/bun/bun.zip"
RUN mv /bun/bun-linux-x64/bun /usr/local/bin/bun
RUN chmod 777 /usr/local/bin/bun


FROM debian:stable-slim
COPY --from=get /usr/local/bin/bun /bin/bun
ADD package.json /app/package.json
RUN cd /app && bun install

WORKDIR /app
CMD ["bun", "run", "/app/html.ts"]

