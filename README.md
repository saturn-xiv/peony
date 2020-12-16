# PEONY

A total free education &amp; translation &amp; ops solution.

## Usage

- [gRPC v1.34.0](https://github.com/grpc/grpc/releases/tag/v1.34.0)

    ```bash
    # PHP
    protoc -I /etc/poney/protos --php_out=php --grpc_out=php --plugin=protoc-gen-grpc=`which grpc_php_plugin` /etc/poney/protos/*.proto
    ```

## Documents

- [grpc](https://grpc.io/docs/languages/cpp/quickstart/)
