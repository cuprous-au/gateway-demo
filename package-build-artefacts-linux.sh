#!/usr/bin/env bash

PUBLISH_VERSION=0.1.0
TARGET="armv7-unknown-linux-musleabihf"

echo
echo "Using PUBLISH_VERSION: $PUBLISH_VERSION for TARGET: $TARGET"
echo

mkdir /tmp/gateway-demo-packaging

cp backend/install /tmp/gateway-demo-packaging
chown 0:0 /tmp/gateway-demo-packaging/install
chmod 700 /tmp/gateway-demo-packaging/install

mkdir /tmp/gateway-demo-packaging/bin
chown 0:0 /tmp/gateway-demo-packaging/bin
chmod 755 /tmp/gateway-demo-packaging/bin

cp target/$TARGET/release/gateway-demo /tmp/gateway-demo-packaging/bin
chown 0:0 /tmp/gateway-demo-packaging/bin/gateway-demo
chmod 700 /tmp/gateway-demo-packaging/bin/gateway-demo

mkdir /tmp/gateway-demo-packaging/etc
chown 0:0 /tmp/gateway-demo-packaging/etc
chmod 755 /tmp/gateway-demo-packaging/etc

mkdir /tmp/gateway-demo-packaging/etc/systemd
chown 0:0 /tmp/gateway-demo-packaging/etc/systemd
chmod 755 /tmp/gateway-demo-packaging/etc/systemd

mkdir /tmp/gateway-demo-packaging/etc/systemd/system
chown 0:0 /tmp/gateway-demo-packaging/etc/systemd/system
chmod 755 /tmp/gateway-demo-packaging/etc/systemd/system

mkdir /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chmod 755 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d

cp backend/override.conf /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/override.conf
chmod 644 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/override.conf

cp backend/pre-start /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/pre-start
chmod 700 /tmp/gateway-demo-packaging/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/pre-start

mkdir /tmp/gateway-demo-packaging/lib
chown 0:0 /tmp/gateway-demo-packaging/lib
chmod 755 /tmp/gateway-demo-packaging/lib

mkdir /tmp/gateway-demo-packaging/lib/gateway-demo
chown 0:0 /tmp/gateway-demo-packaging/lib/gateway-demo
chmod 600 /tmp/gateway-demo-packaging/lib/gateway-demo

gzip --best frontend/dist/*
for file in frontend/dist/*
do
  mv "$file".gz "$file"
done
cp -r frontend/dist/* /tmp/gateway-demo-packaging/lib/gateway-demo
chown 0:0 /tmp/gateway-demo-packaging/lib/gateway-demo/*
chmod 600 /tmp/gateway-demo-packaging/lib/gateway-demo/*

tar -czvf "/tmp/gateway-demo-$TARGET.v$PUBLISH_VERSION.tar.gz" /tmp/gateway-demo-packaging

rm -rf /tmp/gateway-demo-packaging

echo
echo "/tmp/gateway-demo-$TARGET.v$PUBLISH_VERSION.tar.gz created"
echo