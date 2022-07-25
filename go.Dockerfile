FROM golang:1.18.4
WORKDIR /app
COPY . .
RUN GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o server -a -ldflags '-extldflags "-static"' cmd/server.go
CMD ["sh", "-c", "./server"]


