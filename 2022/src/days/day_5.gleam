import gleam/io.{debug}
import gleam/int
import gleam/function.{identity}
import gleam/list.{map}
import gleam/map.{Map}
import gleam/iterator
import gleam/string
import gleam/erlang/process

type Movement {
  Movement(quantity: Int, from: Int, to: Int)
}

pub fn pt_1(input: String) -> Int {
  let #(stack_count, stacks, movements) = load_input(input)
  execute_movements(stacks, movements, list.reverse)
  |> top_of_stacks(stack_count - 1, [])
  |> string.concat
  |> io.println

  0
}

pub fn pt_2(input: String) -> Int {
  let #(stack_count, stacks, movements) = load_input(input)
  execute_movements(stacks, movements, identity)
  |> top_of_stacks(stack_count - 1, [])
  |> string.concat
  |> io.println

  0
}

fn load_input(input: String) -> #(Int, Map(Int, List(String)), List(Movement)) {
  let lines = string.split(input, on: "\n")
  assert Ok(first_line) = list.first(lines)
  let stack_count = { string.length(first_line) + 1 } / 4
  let stacks =
    lines
    |> iterator.from_list
    |> iterator.take_while(string.starts_with(_, "["))
    |> iterator.map(string.to_graphemes)
    |> iterator.to_list

  let parsed = parse_stacks(stacks, stack_count - 1, map.new())

  let movements =
    lines
    |> iterator.from_list
    |> iterator.drop_while(fn(line) { !string.starts_with(line, "move") })
    |> iterator.take_while(string.starts_with(_, "move"))
    |> iterator.map(parse_movement)
    |> iterator.to_list

  #(stack_count, parsed, movements)
}

fn parse_stacks(
  stacks: List(List(String)),
  n: Int,
  parsed: Map(Int, List(String)),
) -> Map(Int, List(String)) {
  case n < 0 {
    True -> parsed
    False -> {
      // Want to parse stack n
      let col =
        map(stacks, row_pluck(_, n))
        |> list.drop_while(fn(x) { x == " " })
      parse_stacks(stacks, n - 1, map.insert(parsed, n, col))
    }
  }
}

fn row_pluck(row: List(String), n: Int) -> String {
  assert Ok(char) = list.at(row, n * 4 + 1)
  char
}

fn parse_movement(line: String) -> Movement {
  case string.split(line, " ") {
    ["move", quantity, "from", from, "to", to] -> {
      assert Ok(quantity) = int.parse(quantity)
      assert Ok(from) = int.parse(from)
      assert Ok(to) = int.parse(to)
      Movement(quantity, from - 1, to - 1)
    }
  }
}

fn execute_movements(
  stacks: Map(Int, List(String)),
  movements: List(Movement),
  process: fn(List(String)) -> List(String),
) -> Map(Int, List(String)) {
  case movements {
    [] -> stacks
    [m, ..ms] ->
      execute_movements(execute_movement(stacks, m, process), ms, process)
  }
}

fn execute_movement(
  stacks: Map(Int, List(String)),
  movement: Movement,
  process: fn(List(String)) -> List(String),
) -> Map(Int, List(String)) {
  assert Ok(from) = map.get(stacks, movement.from)
  assert Ok(to) = map.get(stacks, movement.to)
  let #(to_move, remaining) = list.split(from, movement.quantity)

  // update the stacks
  stacks
  |> map.insert(movement.from, remaining)
  |> map.insert(movement.to, list.append(process(to_move), to))
}

fn top_of_stacks(
  stacks: Map(Int, List(String)),
  i: Int,
  out: List(String),
) -> List(String) {
  case i < 0 {
    True -> out
    False -> {
      assert Ok(stack) = map.get(stacks, i)
      case list.first(stack) {
        Ok(top) -> top_of_stacks(stacks, i - 1, [top, ..out])
        Error(_) -> top_of_stacks(stacks, i - 1, out)
      }
    }
  }
}

fn abort(msg: String) -> Nil {
  io.println(msg)
  process.kill(process.self())
}

fn dump(stacks: Map(Int, List(String)), i: Int) -> Map(Int, List(String)) {
  case i < map.size(stacks) {
    True -> {
      assert Ok(stack) = map.get(stacks, i)
      io.println(int.to_string(i + 1) <> ": " <> string.join(stack, " "))
      dump(stacks, i + 1)
    }
    False -> stacks
  }
}
