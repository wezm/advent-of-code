LIMITS = {
  "red" => 12,
  "green" => 13,
  "blue" => 14,
}

def possible(game)
  game.all? do |draw|
    draw.all? do |count, colour|
      count <= LIMITS[colour]
    end
  end
end

def cubes_power(game)
  max_seen = Hash.new(0)
  game.each do |draw|
    draw.each do |count, colour|
      if count > max_seen[colour]
        max_seen[colour] = count
      end
    end
  end
  LIMITS.keys.each.reduce(1) do |power, colour|
    power * max_seen[colour]
  end
end

possible_ids = []
powers = []

ARGF.each_line do |line|
  game_id = nil
  if line =~ /Game (\d+): (.+)$/
    game_id = $1.to_i
    game = $2.split("; ").map do |draw|
      draw.split(", ").map do |balls|
        count, colour = balls.split
        [count.to_i, colour]
      end
    end
    # p game
    possible_ids << game_id if possible(game)
    powers << cubes_power(game)
  else
    raise "unmatched line"
  end
end

puts possible_ids.sum
puts powers.sum
