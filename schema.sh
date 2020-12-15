#!/bin/sh

set -e

export DATABASE_URL="postgres://postgres:@localhost:5432/peony"

diesel print-schema -o schema_migrations > src/orm/schema.rs
diesel print-schema -o settings > src/settings/schema.rs
diesel print-schema -o locales > src/i18n/schema.rs
diesel print-schema -o notifications attachments policies logs users \
    category_resources categories tag_resources tags votes leave_words friend_links links cards > src/plugins/nut/schema.rs
diesel print-schema -o forum_posts forum_topics > src/plugins/forum/schema.rs

exit 0