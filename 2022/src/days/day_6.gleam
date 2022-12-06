import gleam/io
import gleam/result
import gleam/string
import gleam/list

pub fn pt_1(input: String) -> Int {
  input
  |> tune(4)
}

pub fn pt_2(input: String) -> Int {
  input
  |> tune(14)
}

fn tune(input: String, window: Int) -> Int {
  assert Ok(header) =
    input
    |> string.trim_right
    |> string.to_graphemes
    |> list.index_map(fn(i, el) { #(i + 1, el) })
    |> list.window(window)
    |> list.find(is_unique(_, window))

  assert Ok(index) = list.last(header)
  index.0
}

fn is_unique(chars: List(#(Int, String)), window: Int) -> Bool {
  chars
  |> list.map(fn(pair) { pair.1 })
  |> list.unique
  |> list.length == window
}
