#!/bin/sh

set -e

echo "generate diesel schema..."

export DATABASE_URL="postgres://postgres:@localhost:5432/peony"

# diesel print-schema -o schema_migrations > src/orm/postgresql/schema.rs
# diesel print-schema -o settings > src/settings/schema.rs
# diesel print-schema -o locales > src/i18n/schema.rs
# diesel print-schema -o notifications attachments policies logs users \
#     category_resources categories tag_resources tags votes leave_words friend_links links cards > src/plugins/nut/schema.rs
# diesel print-schema -o forum_posts forum_topics > src/plugins/forum/schema.rs


# cargo install protobuf-codegen
# cargo install grpcio-compiler


echo "generate grpc-rust..."
export RUST_OUT=src/protos
mkdir -p $RUST_OUT
$HOME/.local/bin/protoc --rust_out=$RUST_OUT --grpc_out=$RUST_OUT \
    --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
    $HOME/.local/include/google/protobuf/empty.proto \
    protos/*.proto
# declare -a plugins=(    
#     "nut"
#     "forum"
# )

# for i in "${plugins[@]}"
# do
#     export RUST_OUT=src/plugins/$i/rpc
#     mkdir -p $RUST_OUT
#     $HOME/.local/bin/protoc -I $HOME/.local/include/google/protobuf/ --rust_out=$RUST_OUT --grpc_out=$RUST_OUT --plugin=protoc-gen-grpc=`which grpc_rust_plugin` protos/$i.proto
# done

echo "generate grpc-php..."
export PHP_OUT=tmp/php
if [ -d $PHP_OUT ]
then
    rm -rf $PHP_OUT
fi
mkdir -p $PHP_OUT
$HOME/.local/bin/protoc --php_out=$PHP_OUT --grpc_out=$PHP_OUT \
    --plugin=protoc-gen-grpc=`which grpc_php_plugin` \
    $HOME/.local/include/google/protobuf/empty.proto \
    protos/*.proto

echo 'done.'
exit 0