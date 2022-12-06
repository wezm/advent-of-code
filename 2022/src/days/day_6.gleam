import gleam/io
import gleam/result
import gleam/string
import gleam/list

pub fn pt_1(input: String) -> Int {
  assert Ok(header) =
    input
    |> string.trim_right
    |> string.to_graphemes
    |> list.index_map(fn(i, el) { #(i + 1, el) })
    |> list.window(4)
    |> list.find(is_unique)

  assert Ok(index) = list.last(header)
  index.0
}

pub fn pt_2(input: String) -> Int {
  todo
}

fn is_unique(chars: List(#(Int, String))) -> Bool {
  chars
  |> list.map(fn(pair) { pair.1 })
  |> list.unique
  |> list.length == 4
}
