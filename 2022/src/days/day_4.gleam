import gleam/int
import gleam/list.{map}
import gleam/iterator
import gleam/string

type Range {
  Range(start: Int, end: Int)
}

pub fn pt_1(input: String) -> Int {
  string.split(input, on: "\n")
  |> iterator.from_list
  |> iterator.map(parse_line)
  |> iterator.filter(fully_contains)
  |> iterator.fold(0, fn(sum, _) { sum + 1 })
}

pub fn pt_2(input: String) -> Int {
  todo
}

fn parse_line(line: String) -> #(Range, Range) {
  assert [a, b] =
    string.split(line, ",")
    |> map(parse_range)
  #(a, b)
}

fn parse_range(str: String) -> Range {
  assert [start, end] =
    string.split(str, "-")
    |> map(parse_int)
  Range(start, end)
}

fn parse_int(str: String) -> Int {
  assert Ok(i) = int.parse(str)
  i
}

fn fully_contains(pair: #(Range, Range)) -> Bool {
  let a = pair.0
  let b = pair.1
  b.start >= a.start && b.end <= a.end || a.start >= b.start && a.end <= b.end
}
