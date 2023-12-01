#!/bin/sh

awk -f day1.2.awk $1 | tr -d a-z | grep -o '^.' > first
rev $1 | awk -f day1.2.rev.awk | rev | tr -d a-z | grep -o '.$' > last

paste -d '\0' first last | \
  awk 'BEGIN { sum = 0 } { sum += $0 } END { print sum }'
