#!/bin/sh

tr -d a-z < $1 | grep -o '^.' > first
tr -d a-z < $1 | grep -o '.$' > last
paste -d '\0' first last | \
  awk 'BEGIN { sum = 0 } { sum += $0 } END { print sum }'
