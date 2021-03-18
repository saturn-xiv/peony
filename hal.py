#!/usr/bin/env python
# -*- coding: utf-8 -*-


import logging
import getopt
import argparse
import time
import _thread
import sys
import toml
import zmq
import serial


class Tty:
    def __init__(self, name, matchers):
        logging.info("open serial port %s", name)
        self.port = serial.Serial("/dev/%s" % name, timeout=1)
        self.matchers = matchers
        self.buffer = bytearray(0)

    def write(self, message):
        self.port.write(message)

    def listen(self, publisher):
        while True:
            try:
                self.loop(publisher)
            except Exception as e:
                logging.error("%s: %s", self.port.name, e)

    def loop(self, publisher):
        line = self.port.read(1 << 8)
        if len(line) == 0:
            return
        logging.debug("%s read %d bytes", self.port.name, len(line))
        self.buffer += line
        msg = self.buffer.decode()
        logging.debug("%s buffer[%d, %d]: %s", self.port.name, len(
            msg), len(self.buffer),  msg)
        for it in self.matchers:
            begin = msg.find(it["begin"])
            if begin == -1:
                continue
            if begin != 0:
                logging.warning("%s miss part: %s", self.port.name, msg)
            end = msg.find(it["end"])
            if end == -1:
                continue
            end += len(it["end"])
            cur = msg[begin:end]
            self.buffer = self.buffer[end:]
            logging.info("%s match: %s", self.port.name, cur)
            logging.debug("%s split: %s", self.port.name, msg[:end])
            publisher.send_json({"tty": self.port.name, "message": cur})


def launch(args):
    logging.debug("%s", args)
    logging.info("start hal daemon %s mode",
                 "emulator" if args.emulator else "hardware")
    logging.debug("load config from %s", args.config)
    cfg = toml.load(args.config)
    logging.debug("%s", cfg)

    p_ctx = zmq.Context.instance()
    publisher = p_ctx.socket(zmq.PUB)
    publisher.bind("tcp://*:%d" % cfg["port"])

    tty_ports = dict()
    for it in cfg["tty"]:
        name = it["name"]
        tty_ports[name] = Tty(name, it["matchers"])

    logging.info("start consumer %s" % cfg["name"])
    c_ctx = zmq.Context()
    consumer = c_ctx.socket(zmq.PULL)
    consumer.connect("ipc:///tmp/%s.sck" % cfg["name"])
    time.sleep(1)

    for name, tty in tty_ports.items():
        logging.info("start tty %s publisher", name)
        _thread.start_new_thread(tty.listen, (publisher,))

    try:
        while True:
            payload = consumer.recv_json()
            logging.info("receive: %s", payload)
            if payload["tty"]:
                tty = tty_ports[payload["tty"]]
                if not tty:
                    logging.error("can't find tty %s", payload["name"])
                    continue
                tty.write(payload["message"])
            elif payload["audio"]:
                logging.warning("TODO audio")
            elif payload["tts"]:
                logging.warning("TODO tts")
            else:
                logging.error("unknown payload")
    except KeyboardInterrupt:
        sys.exit()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='HAL daemon.')
    parser.add_argument('-d', '--debug', action=argparse.BooleanOptionalAction)
    parser.add_argument('-e', '--emulator',
                        action=argparse.BooleanOptionalAction)
    parser.add_argument('-c', '--config', default="config.toml")
    args = parser.parse_args()
    logging.basicConfig(level=(logging.DEBUG if args.debug else logging.INFO))
    launch(args)
