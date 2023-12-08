START = "AAA"
FINISH = "ZZZ"

class Node
  attr_reader :name, :left, :right

  def initialize(name, left, right)
    @name = name
    @left = left
    @right = right
  end
end

raw_instructions, raw_nodes = ARGF.read.split("\n\n")

nodes = Hash.new
raw_nodes.each_line do |line|
  next if line.empty?

  if line =~ /^([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)$/
    nodes[$1] = Node.new($1, $2, $3)
  else
    throw "unmatched line #{line}"
  end
end

node = nodes[START]
instructions = raw_instructions.split("").cycle
steps = 0
while node.name != FINISH
  steps += 1
  instruction = instructions.next
  case instruction
  when "L"
    node = nodes[node.left]
  when "R"
    node = nodes[node.right]
  else
    throw "unexpected instruction: '#{instruction}'"
  end
end

puts "Part 1: #{steps}"
