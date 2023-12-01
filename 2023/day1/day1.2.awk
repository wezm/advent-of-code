BEGIN {
  map["one"] = 1
  map["two"] = 2
  map["three"] = 3
  map["four"] = 4
  map["five"] = 5
  map["six"] = 6
  map["seven"] = 7
  map["eight"] = 8
  map["nine"] = 9
}

{
  pos = match($0, "one|two|three|four|five|six|seven|eight|nine")
  if (pos != 0) {
    numeral = substr($0, RSTART, RLENGTH)
    digit = map[numeral]
    sub(numeral, digit)
  }
  print
}

