# Docker

- Usage

```bash
# first time
podman run --name samoyed -it --userns=keep-id --user=$(id -ur):$(id -gr) --network host --events-backend=file -v $PWD:/workspace:z CODE
# next time
podman start -i -a --events-backend=file CODE
```

## Buildroot

```bash
make raspberrypi3_defconfig
```

## Os & Image
  
| Board     |  Model   |                                                              Install OS | Default User  | Docker Code |
| --------- | :------: | ----------------------------------------------------------------------: | ------------- | ----------- |
| Raspberry |  3b+,4   |                      [Raspbian](https://www.raspberrypi.org/downloads/) | pi:raspberry  | bionic      |
| Orange    | zero,one |                            [Armbian](https://www.armbian.com/download/) | root:1234     | bionic      |
| Nano      |   duo2   | [Friendly Core](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2) | pi:pi root:fa | xenial      |
| Aws       |  x86_64  |                                                                         |               | bionic      |
