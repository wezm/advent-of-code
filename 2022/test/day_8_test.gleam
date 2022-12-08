import gleeunit
import gleeunit/should
import days/day_8

pub fn main() {
  gleeunit.main()
}

pub fn scan_max_test() {
  day_8.scan_max([2, 5, 5, 1, 2])
  |> should.equal([-1, 2, 5, 5, 5])
}

// 30373
// 25512
// 65332
// 33549
// 35390
pub fn pt1_test() {
  day_8.pt_1("30373\n25512\n65332\n33549\n35390")
  |> should.equal(21)
}

pub fn pt2_test() {
  day_8.pt_2("30373\n25512\n65332\n33549\n35390")
  |> should.equal(8)
}
