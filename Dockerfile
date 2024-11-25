FROM debian:stable-20241111-slim

RUN apt-get update && apt-get install -y ca-certificates curl
WORKDIR /app
COPY euro-usd-exporter /app/
EXPOSE 8080

CMD ["/app/euro-usd-exporter"]