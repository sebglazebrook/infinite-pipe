FROM sebglazebrook/rust-nightly:latest

RUN apt-get update && apt-get install --yes gcc make libncurses5-dev
