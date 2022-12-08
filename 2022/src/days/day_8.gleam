import gleam/int
import gleam/iterator.{zip}
import gleam/string
import gleam/list
import gleam/option.{None, Some}

pub fn pt_1(input: String) -> Int {
  let trees_by_row =
    input
    |> string.split("\n")
    |> list.map(parse_row)

  let trees_by_col = list.transpose(trees_by_row)

  // Generate grids of max values from left, right, top, bottom
  let left =
    list.map(trees_by_row, scan_max)
    |> iterator.from_list

  let right =
    trees_by_row
    |> list.map(list.reverse)
    |> list.map(scan_max)
    |> list.map(list.reverse)
    |> iterator.from_list

  let top =
    list.map(trees_by_col, scan_max)
    |> list.transpose
    |> iterator.from_list

  let bottom =
    trees_by_col
    |> list.map(list.reverse)
    |> list.map(scan_max)
    |> list.map(list.reverse)
    |> list.transpose
    |> iterator.from_list

  trees_by_row
  |> iterator.from_list
  |> zip(left)
  |> zip(right)
  |> zip(top)
  |> zip(bottom)
  |> iterator.fold(0, process_row)
}

pub fn pt_2(input: String) -> Int {
  todo
}

fn parse_row(row: String) -> List(Int) {
  row
  |> string.to_graphemes
  |> list.map(parse_int)
}

fn parse_int(text: String) -> Int {
  assert Ok(i) = int.parse(text)
  i
}

pub fn scan_max(numbers: List(Int)) -> List(Int) {
  numbers
  |> list.scan(
    #(None, 0),
    fn(acc, tree) {
      case acc {
        // Edge tree
        #(None, _) -> #(Some(tree), -1)
        // First tree in from edge
        #(Some(prev), -1) -> #(Some(tree), prev)
        // Other tree
        #(Some(prev), max) ->
          case prev > max {
            True -> #(Some(tree), prev)
            False -> #(Some(tree), max)
          }
      }
    },
  )
  |> list.map(fn(tup) { tup.1 })
}

// That's not a type, now _this_ is a type lol
fn process_row(
  count: Int,
  rows: #(#(#(#(List(Int), List(Int)), List(Int)), List(Int)), List(Int)),
) -> Int {
  let #(#(#(#(trees, left), right), top), bottom) = rows

  trees
  |> iterator.from_list
  |> zip(iterator.from_list(left))
  |> zip(iterator.from_list(right))
  |> zip(iterator.from_list(top))
  |> zip(iterator.from_list(bottom))
  |> iterator.fold(count, count_if_visible)
}

fn count_if_visible(
  count: Int,
  record: #(#(#(#(Int, Int), Int), Int), Int),
) -> Int {
  let #(#(#(#(tree, left), right), top), bottom) = record

  case
    [left, right, top, bottom]
    |> list.any(fn(max) { tree > max })
  {
    True -> count + 1
    False -> count
  }
}
