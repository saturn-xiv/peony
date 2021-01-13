# Docker

## Usage

```bash
docker run --rm -it --network host -v `pwd`:/workspace peony-CODE
```

## Os & Image
  
| Board     |  Model   |                                                              Install OS | Docker Image  | Default User  | Docker Code |
| --------- | :------: | ----------------------------------------------------------------------: | ------------- | ------------- | ----------- |
| Raspberry |  3b+,4   |                      [Raspbian](https://www.raspberrypi.org/downloads/) | raspbian      |               |             |
| Orange    | zero,one |                            [Armbian](https://www.armbian.com/download/) |               | armbian       |             |
| Nano      |   duo2   | [Friendly Core](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2) | friendly-core | pi:pi root:fa | xenial      |
| Aws       |  x86_64  |                                                                         | bionic        |               |             |
