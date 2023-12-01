#!/bin/zsh

paste -d '\0' <(tr -d a-z < $1 | rg -o ^.) <(tr -d a-z < $1 | rg -o .$) | \
  awk 'BEGIN { sum = 0 } { sum += $0 } END { print sum }'
