# Docker

## Usage

```bash
podman run --rm -it --userns=keep-id --user=$(id -ur):$(id -gr) --network host -v `pwd`:/workspace:z peony-CODE
```

## Os & Image
  
| Board     |  Model   |                                                              Install OS | Default User  | Docker Code |
| --------- | :------: | ----------------------------------------------------------------------: | ------------- | ----------- |
| Raspberry |  3b+,4   |                      [Raspbian](https://www.raspberrypi.org/downloads/) | pi:raspberry  | bionic      |
| Orange    | zero,one |                            [Armbian](https://www.armbian.com/download/) | root:1234     | bionic      |
| Nano      |   duo2   | [Friendly Core](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2) | pi:pi root:fa | xenial      |
| Aws       |  x86_64  |                                                                         |               | bionic      |
