#!/bin/sh

set -e

# https://create-react-app.dev/docs/adding-typescript/
# react-quill@beta react-mde emoji-mart google-map-react qrcode.react react-copy-to-clipboard draft-js
# @fluentui/react recharts
npm install --save \
    js-cookie @types/js-cookie jwt-decode \
    dinero @types/dinero.js \
    moment moment-timezone \
    react-markdown @uiw/react-md-editor \
    react-intl \
    redux react-redux @types/react-redux \
    react-router-dom @types/react-router-dom \
    react-syntax-highlighter @types/react-syntax-highlighter \
    grpc-web \
    @material-ui/core @material-ui/icons fontsource-roboto

exit 0