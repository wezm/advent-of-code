NUM = /\d+/
SYM = /[^.]/

def part1(input)
  part_numbers = []

  input.each_with_index do |line, i|
    offset = 0
    while m = NUM.match(line, offset)
      start = m.begin(0)
      finish = m.end(0)
      offset = finish
      span = [0, start - 1].max..finish

      #p i.to_s, m[0]
      above = input[i - 1][span] if i > 0
      below = input[i + 1][span] if i < input.length - 1
      left = line[start - 1] if start > 0
      right = line[finish]
      #p [line[span], above, below, left, right]
      #p [i, m[0], start, finish, line[span]]

      if [above, below, left, right].compact.any? { |x| x.match?(SYM) }
          part_numbers << m[0].to_i
      end
    end
  end

  part_numbers.sum
end

def part2(input)
  # todo
end

input = ARGF.readlines.map(&:rstrip)
puts part1(input)
puts part2(input)
