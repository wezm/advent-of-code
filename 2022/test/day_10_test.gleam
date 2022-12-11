import gleeunit
import gleeunit/should
import days/day_10.{Trace}
import gleam/erlang/file
import gleam/string
import gleam/list

pub fn main() {
  gleeunit.main()
}

pub fn execute_test() {
  ["noop", "addx 3", "addx -5"]
  |> list.fold([Trace(1, 0)], day_10.execute)
  |> list.reverse
  |> should.equal([Trace(1, 0), Trace(1, 1), Trace(4, 3), Trace(-1, 5)])
}

pub fn pt1_test() {
  assert Ok(input) = file.read("input/day_10_pt1_test.txt")
  input
  |> string.trim_right
  |> day_10.pt_1
  |> should.equal(13140)
}
