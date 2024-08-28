#!/usr/bin/env bash

PUBLISH_VERSION=0.1.0
TARGET="armv7-unknown-linux-musleabihf"

echo
echo "Using PUBLISH_VERSION: $PUBLISH_VERSION for TARGET: $TARGET"
echo

mkdir /tmp/gateway-demo

cp backend/install /tmp/gateway-demo
chown 0:0 /tmp/gateway-demo/install
chmod 700 /tmp/gateway-demo/install

mkdir /tmp/gateway-demo/bin
chown 0:0 /tmp/gateway-demo/bin
chmod 755 /tmp/gateway-demo/bin

cp target/$TARGET/release/gateway-demo /tmp/gateway-demo/bin
chown 0:0 /tmp/gateway-demo/bin/gateway-demo
chmod 700 /tmp/gateway-demo/bin/gateway-demo

mkdir /tmp/gateway-demo/etc
chown 0:0 /tmp/gateway-demo/etc
chmod 755 /tmp/gateway-demo/etc

mkdir /tmp/gateway-demo/etc/systemd
chown 0:0 /tmp/gateway-demo/etc/systemd
chmod 755 /tmp/gateway-demo/etc/systemd

mkdir /tmp/gateway-demo/etc/systemd/system
chown 0:0 /tmp/gateway-demo/etc/systemd/system
chmod 755 /tmp/gateway-demo/etc/systemd/system

mkdir /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chmod 755 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d

cp backend/override.conf /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/override.conf
chmod 644 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/override.conf

cp backend/pre-start /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d
chown 0:0 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/pre-start
chmod 700 /tmp/gateway-demo/etc/systemd/system/systemd-nspawn@gateway-demo.service.d/pre-start

mkdir /tmp/gateway-demo/lib
chown 0:0 /tmp/gateway-demo/lib
chmod 755 /tmp/gateway-demo/lib

mkdir /tmp/gateway-demo/lib/gateway-demo
chown 0:0 /tmp/gateway-demo/lib/gateway-demo
chmod 600 /tmp/gateway-demo/lib/gateway-demo

gzip --best frontend/dist/*
for file in frontend/dist/*
do
  mv "$file".gz "$file" 2> /dev/null || true
done
cp -r frontend/dist/* /tmp/gateway-demo/lib/gateway-demo
chown 0:0 /tmp/gateway-demo/lib/gateway-demo/*
chmod 600 /tmp/gateway-demo/lib/gateway-demo/*

mkdir /tmp/gateway-demo/usr
chown 0:0 /tmp/gateway-demo/usr
chmod 755 /tmp/gateway-demo/usr

mkdir /tmp/gateway-demo/usr/bin
chown 0:0 /tmp/gateway-demo/usr/bin
chmod 755 /tmp/gateway-demo/usr/bin

tar -czvf "/tmp/gateway-demo-$TARGET.v$PUBLISH_VERSION.tar.gz" -C /tmp gateway-demo

rm -rf /tmp/gateway-demo

echo
echo "/tmp/gateway-demo-$TARGET.v$PUBLISH_VERSION.tar.gz created"
echo