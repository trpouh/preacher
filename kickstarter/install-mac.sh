#!/bin/zsh
TAG=0.0.3
URL=https://github.com/trpouh/preacher/releases/download/${TAG}/preacher_${TAG}_x86_64-apple-darwin.zip

echo "[preacher] installing preacher v${TAG} - amen!";
echo "[preacher] download binary from: ${URL}"

curl -L $URL --output preacher.zip

echo "[preacher] unzipping preacher.zip";

unzip preacher.zip

echo "[preacher] cleaning up..."

rm preacher.zip
rm README.md

echo "[preacher] creating your first sermon under 'sermon.yaml' have a look at it!"

cat <<EOT >> sermon.yaml
psalms:
  - type: Hello
EOT

echo "[preacher] start your first sermon with './preacher'"

