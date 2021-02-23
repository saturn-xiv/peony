# PEONY

A total free education &amp; translation &amp; ops solution.

## Development

```bash
cargo install grpcio-compiler protobuf-codegen diesel_cli
```

## Usage

```bash
cd docker
docker build -t peony .
cd ..
docker run --rm -it --network host -v `pwd`:/workspace peony
> ./docker/deb.sh amd64 # or armhf
```

## Documents

### Tools

- [SDKMAN!](https://sdkman.io/usage)
- [Node Version Manager](https://github.com/nvm-sh/nvm)
- [Oh My Zsh](https://github.com/ohmyzsh/ohmyzsh)
- [Dockerfile reference](https://docs.docker.com/engine/reference/builder/)
- [Visual Studio Code](https://code.visualstudio.com/Download)

### Background

- [FlatBuffers](https://google.github.io/flatbuffers/flatbuffers_support.html)

- [PostgreSQL](https://www.postgresql.org/docs/current/)
- [Redis](https://redis.io/commands)
- [Eclipse Mosquitto](https://mosquitto.org/documentation/)
- [RabbitMQ](https://www.rabbitmq.com/admin-guide.html)
- [Elasticsearch](https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html)

### Frontend

- [Bootstrap](https://getbootstrap.com/)
- [Bulma: the modern CSS framework that just works.](https://bulma.io/)
- [Material Icons Guide](https://google.github.io/material-design-icons/)
- [Moment.js](https://momentjs.com/)
- [Marked](https://github.com/markedjs/marked)

- [Vuetify](https://vuetifyjs.com/en/getting-started/installation/)
