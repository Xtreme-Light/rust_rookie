#!/bin/bash
if [[ -f ~/RustProjects/exa-as-we-see/target/debug/exa-as-we-see ]]; then
  target/debug/clap-exa
else
  echo -e "clap-exa"
  echo -e "Run \033[32;1mb\033[0m or \033[32;1mbuild-exa\033[0m to create it"
fi
