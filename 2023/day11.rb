MULTIPLIER = ENV["MULTIPLIER"].to_i || 2
input = ARGF.read.strip
by_row = input.lines.map(&:rstrip).map { |line| line.split("") }

def needs_expand(data)
  expand = []
  data.each_with_index { |row, i| expand << i if row.all? { |c| c == "." || c == "," } }
  expand
end

def print_cosmos(cosmos)
  cosmos.each do |line|
    puts line.join("")
  end
end

# do expansion
row_expand = needs_expand(by_row)
row_expand.reverse.each do |i|
  by_row[i] = ("," * by_row[i].length).split("") # add expansion marker
end

by_column = by_row.transpose
col_expand = needs_expand(by_column)
col_expand.reverse.each do |i|
  by_column[i] = ("," * by_column[i].length).split("") # add expansion marker
end

cosmos = by_column.transpose
print_cosmos(cosmos)

# find galaxies
galaxies = []
(0...cosmos.first.length).each do |x|
  (0...cosmos.length).each do |y|
    galaxies << [x, y] if cosmos[y][x] == "#"
  end
end

def galaxy_distance(a, b)
  (a[0] - b[0]).abs + (a[1] - b[1]).abs
end

def cosmos_at(x, y, cosmos)
  cosmos[y][x]
end

distances = galaxies.combination(2).map do |combo|
  a, b = combo.sort
  distance = galaxy_distance(a, b)

  # see how many millions are crossed in x
  x_millions = (a[0]..b[0]).filter_map do |x|
    loc = cosmos_at(x, a[1], cosmos)
    loc if loc == ","
  end

  # now y component
  range = if a[1] > b[1]
      b[1]..a[1]
    else
      a[1]..b[1]
    end
  y_millions = range.filter_map do |y|
    loc = cosmos_at(b[0], y, cosmos)
    loc if loc == ","
  end

  distance + ((x_millions.length + y_millions.length) * MULTIPLIER) - x_millions.length - y_millions.length
end

puts "Sum of distances with multiplier #{MULTIPLIER}: #{distances.sum}"
