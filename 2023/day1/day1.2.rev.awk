BEGIN {
  map["eno"] = 1
  map["owt"] = 2
  map["eerht"] = 3
  map["ruof"] = 4
  map["evif"] = 5
  map["xis"] = 6
  map["neves"] = 7
  map["thgie"] = 8
  map["enin"] = 9
}

{
  pos = match($0, "enin|thgie|neves|xis|evif|ruof|eerht|owt|eno")
  if (pos != 0) {
    numeral = substr($0, RSTART, RLENGTH)
    digit = map[numeral]
    sub(numeral, digit)
  }
  print
}

