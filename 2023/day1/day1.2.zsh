#!/bin/zsh

paste -d '\0' \
  <(awk -f day1.2.awk $1 | tr -d a-z | rg -o '^.')   \
  <(rev $1 | awk -f day1.2.rev.awk | rev | tr -d a-z | rg -o '.$') | \
  awk 'BEGIN { sum = 0 } { sum += $0 } END { print sum }'
