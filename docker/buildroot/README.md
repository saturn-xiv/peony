# Buildroot

```bash
podman run --rm -it --userns=keep-id --user=$(id -ur):$(id -gr) --network host -v `pwd`:/workspace:z peony-buildroot
> cd $HOME/buildroot
> make O=/workspace/tmp/raspberry-3 raspberry-3 # orange-0 nano-duo2
> cd /workspace/tmp/raspberry-3
> make menuconfig # optional
> make
> ls $HOME/x-tools
```

## Documents

- [crosstool-NG](https://crosstool-ng.github.io/docs/)
- [GNU-A Downloads](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads)
- [The Buildroot user manual](https://buildroot.org/downloads/manual/manual.html)
- [Raspberry pi - Kernel building](https://www.raspberrypi.org/documentation/linux/kernel/building.md)
- [Orange Pi Zero](http://www.orangepi.org/orangepizero/)
- [Nano pi Duo2](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2)
- [Raspberry Pi 3 Model B+](https://www.raspberrypi.org/products/raspberry-pi-3-model-b-plus/)
