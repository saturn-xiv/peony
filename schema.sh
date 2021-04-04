#!/bin/sh

set -e

echo "generate diesel schema..."

. ./.env

diesel print-schema -o settings > src/settings/schema.rs
diesel print-schema -o locales > src/i18n/schema.rs
diesel print-schema -o notifications attachments policies logs users \
    category_resources categories tag_resources tags votes leave_words friend_links links cards > src/plugins/nut/schema.rs
diesel print-schema -o forum_posts forum_topics > src/plugins/forum/schema.rs
diesel print-schema -o ops_crawler_logs > src/plugins/ops/crawler/schema.rs
diesel print-schema -o ops_cron_tasks > src/plugins/ops/cron/schema.rs


echo "generate flatbuffers schema..."
flatc --rust --filename-suffix '' -o src/protos/ protos/*.fbs

echo "format code"
cargo fmt

echo 'done.'
exit 0