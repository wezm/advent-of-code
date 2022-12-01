import gleam/string
import gleam/int
import gleam/list
import gleam/io

pub fn pt_1(input: String) -> Int {
  assert Ok(max) =
    string.split(input, on: "\n")
    |> aggregate_calories([0])
    |> list.reduce(int.max)
  max
}

pub fn pt_2(input: String) -> Int {
  todo
}

fn aggregate_calories(lines: List(String), sums: List(Int)) -> List(Int) {
  case lines {
    [] -> sums
    ["", ..rest] -> aggregate_calories(rest, [0, ..sums])
    [num, ..rest] -> {
      assert Ok(calories) = int.parse(num)
      inc(calories, sums)
      |> aggregate_calories(rest, _)
    }
  }
}

fn inc(num: Int, sums: List(Int)) -> List(Int) {
  case sums {
    // should be unreachable
    [] -> []
    [first, ..rest] -> [first + num, ..rest]
  }
}
