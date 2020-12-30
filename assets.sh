#!/bin/sh

set -e

# npm install --save material-design-icons marked moment-timezone moment bootstrap@next

# react-quill@beta react-mde emoji-mart google-map-react qrcode.react react-copy-to-clipboard
# @ant-design/charts antd
npm install --save \
    js-cookie @types/js-cookie \
    jwt-decode \
    dinero @types/dinero.js \
    moment moment-timezone \
    draft-js react-markdown remark-gfm \
    react-intl \
    redux react-redux @types/react-redux \
    react-router-dom @types/react-router-dom \
    react-syntax-highlighter @types/react-syntax-highlighter \
    grpc-web \
    @fontsource/roboto material-design-icons \
    formik react-hook-form \
    @material-ui/core@next @material-ui/icons@next @emotion/react @emotion/styled

echo 'Done.'

exit 0