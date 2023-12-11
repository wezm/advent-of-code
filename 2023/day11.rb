input = ARGF.read.strip
by_row = input.lines.map(&:rstrip).map { |line| line.split("") }

def needs_expand(data)
  expand = []
  data.each_with_index { |row, i| expand << i if row.all? { |c| c == "." } }
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
  by_row.insert(i, by_row[i].dup)
end

by_column = by_row.transpose
col_expand = needs_expand(by_column)
col_expand.reverse.each do |i|
  by_column.insert(i, by_column[i].dup)
end

cosmos = by_column.transpose

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

puts "Part 1: #{galaxies.combination(2).map { |a, b| galaxy_distance(a, b) }.sum}"
