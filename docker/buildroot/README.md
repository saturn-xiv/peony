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

- [The Buildroot user manual](https://buildroot.org/downloads/manual/manual.html)
- [Nano pi Duo2](http://wiki.friendlyarm.com/wiki/index.php/NanoPi_Duo2)
- [Raspberry pi](https://www.raspberrypi.org/documentation/linux/kernel/building.md)
