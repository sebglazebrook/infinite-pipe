FROM sebglazebrook/rust-nightly

RUN apt-get update && apt-get install --yes gcc make libncurses5-dev
