#!/bin/bash -u

VERSION_NUMBER=${1}

docker build -t infinite-pipe-debian-pkg-builder -f packagers/debian/Dockerfile .

CONTAINER_ID=$(docker create -e VERSION_NUMBER="$VERSION_NUMBER" infinite-pipe-debian-pkg-builder)

docker start -ai ${CONTAINER_ID}

docker cp ${CONTAINER_ID}:/tmp/infinite-pipe_${VERSION_NUMBER}.deb .

mkdir -p releases/$VERSION_NUMBER/debian && mv infinite-pipe_${VERSION_NUMBER}.deb releases/$VERSION_NUMBER/debian/
