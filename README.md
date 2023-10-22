# Health (aka System Design homework №1)

## Main

The simple HTTP-server is implemented in Rust + Actix

Endpoints:
- `/`: returns hello
- `/health/`: returns health state


## Start

### Compose

The simpliest way to launch the app is to run
```bash
docker-compose up --build -d
```

### Build from scratch

```bash
docker build --tag=health .
docker run -p 8000:8000 health
```

### Pull image from Hub
```bash
docker run -i -t -p 8000:8000 offluck/healthy:latest
```
