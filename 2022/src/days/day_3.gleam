import gleam/int
import gleam/set
import gleam/list.{map}
import gleam/string

type Rucksack =
  #(List(String), List(String))

pub fn pt_1(input: String) -> Int {
  string.split(input, on: "\n")
  |> list.map(string.to_graphemes)
  |> list.map(split_in_half)
  |> list.map(common_item)
  |> list.map(priority)
  |> int.sum
}

pub fn pt_2(input: String) -> Int {
  todo
}

fn split_in_half(l: List(String)) -> Rucksack {
  list.split(l, at: list.length(l) / 2)
}

fn common_item(rucksack: Rucksack) -> String {
  let one = set.from_list(rucksack.0)
  let two = set.from_list(rucksack.1)
  let inter = set.intersection(one, two)
  assert 1 = set.size(inter)
  assert Ok(common) =
    inter
    |> set.to_list
    |> list.first
  common
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// I can't find a way to get a codepoint or match on a range with Gleam so this
// will have to do.
fn priority(item: String) -> Int {
  // case <<item:utf8>> {
  //   <<a:utf8_codepoint>> -> a
  // }
  case item {
    "a" -> 1
    "b" -> 2
    "c" -> 3
    "d" -> 4
    "e" -> 5
    "f" -> 6
    "g" -> 7
    "h" -> 8
    "i" -> 9
    "j" -> 10
    "k" -> 11
    "l" -> 12
    "m" -> 13
    "n" -> 14
    "o" -> 15
    "p" -> 16
    "q" -> 17
    "r" -> 18
    "s" -> 19
    "t" -> 20
    "u" -> 21
    "v" -> 22
    "w" -> 23
    "x" -> 24
    "y" -> 25
    "z" -> 26
    "A" -> 27
    "B" -> 28
    "C" -> 29
    "D" -> 30
    "E" -> 31
    "F" -> 32
    "G" -> 33
    "H" -> 34
    "I" -> 35
    "J" -> 36
    "K" -> 37
    "L" -> 38
    "M" -> 39
    "N" -> 40
    "O" -> 41
    "P" -> 42
    "Q" -> 43
    "R" -> 44
    "S" -> 45
    "T" -> 46
    "U" -> 47
    "V" -> 48
    "W" -> 49
    "X" -> 50
    "Y" -> 51
    "Z" -> 52
  }
}
