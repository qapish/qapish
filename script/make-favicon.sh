#!/bin/bash

magick \
    web/public/favicon-32.png \
    -define icon:auto-resize=32,16 \
    web/public/favicon.ico
