#!/usr/bin/env bash

# This script is used to update the descriptors used by this Substreams.

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

main() {
  pushd "$ROOT" &> /dev/null

  # By using fixed local version(s), we achieve two goals:
  #  - We have a fixed deterministic version of the descriptor, which is important for reproducibility
  #  - We avoid Buf API rate limits as updating is not done as often as when packaging a Substreams
  wget -O firehose-stellar.binpb "https://buf.build/streamingfast/firehose-stellar/descriptor/v1.0.0"
}

main "$@"