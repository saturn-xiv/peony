# Docker

## Usage

```bash
docker run --rm -it --network host -v `pwd`:/workspace peony-CODE
```

## Os & Image
  
| Board     |  Model   |                                                              Install OS | Default User  | Docker Code |
| --------- | :------: | ----------------------------------------------------------------------: | ------------- | ----------- |
| Raspberry |  3b+,4   |                      [Raspbian](https://www.raspberrypi.org/downloads/) | pi:raspberry  |             |
| Orange    | zero,one |                            [Armbian](https://www.armbian.com/download/) | root:1234     |             |
| Nano      |   duo2   | [Friendly Core](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2) | pi:pi root:fa | xenial      |
| Aws       |  x86_64  |                                                                         |               | bionic      |
