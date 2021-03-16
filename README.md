# PEONY

A total free education &amp; translation &amp; ops solution.

## Usage

```bash
# for ubuntu
sudo apt-get install crun podman buildsh
# for archlinux
sudo pacman -S podman buildah
cd docker/ubuntu
# build docker images
./build.sh
cd ..
# first run image CODE container NAME
podman run --name NAME -it --userns=keep-id --user=$(id -ur):$(id -gr) --network host --events-backend=file -v $PWD:/workspace:z peony-CODE
# build package
> ./docker/deb.sh amd64 # or armhf
```

## Documents

### Tools

- [SDKMAN!](https://sdkman.io/usage)
- [Node Version Manager](https://github.com/nvm-sh/nvm)
- [Oh My Zsh](https://github.com/ohmyzsh/ohmyzsh)
- [Podman](https://www.redhat.com/sysadmin/podman-windows-wsl2)
- [WSL2 install](https://docs.microsoft.com/en-us/windows/wsl/install-win10)
- [WSL2 config](https://docs.microsoft.com/en-us/windows/wsl/wsl-config)
- [Dockerfile reference](https://docs.docker.com/engine/reference/builder/)
- [Visual Studio Code](https://code.visualstudio.com/Download)
- [Ansible](https://docs.ansible.com/ansible/latest/index.html)

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
