#!/bin/sh

set -e

npm install --save material-design-icons marked moment-timezone moment bootstrap@next

# https://create-react-app.dev/docs/adding-typescript/
# react-quill@beta react-mde emoji-mart google-map-react qrcode.react react-copy-to-clipboard
# @ant-design/charts
cd dashboard && npm install --save \
    js-cookie @types/js-cookie \
    jwt-decode @types/jwt-decode \
    dinero @types/dinero.js \
    moment moment-timezone \
    draft-js react-markdown remark-gfm \
    react-intl \
    redux react-redux @types/react-redux \
    react-router-dom @types/react-router-dom \
    react-syntax-highlighter @types/react-syntax-highlighter \
    grpc-web antd 

exit 0