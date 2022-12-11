import gleam/io
import gleam/int
import gleam/list
import gleam/string
import gleam/option.{None, Option, Some}
import gleam/erlang/process

pub type Trace {
  Trace(x: Int, cycles: Int)
}

pub fn pt_1(input: String) -> Int {
  let want = [20, 60, 100, 140, 180, 220]

  string.split(input, "\n")
  |> list.fold([Trace(1, 0)], execute)
  |> list.reverse
  |> list.window_by_2
  |> collect(want, [])
  |> list.reverse
  |> list.zip(want)
  |> list.map(signal_strength)
  |> int.sum
}

pub const display_width = 40

pub fn pt_2(input: String) -> Int {
  string.split(input, "\n")
  |> list.fold([Trace(1, 0)], execute)
  |> list.reverse
  |> list.window_by_2
  |> list.fold([], inflate)
  |> list.fold([], render)
  |> list.reverse
  |> list.sized_chunk(display_width)
  |> list.map(string.join(_, ""))
  |> string.join("\n")
  |> io.println
  2
}

pub fn execute(trace: List(Trace), instruction: String) -> List(Trace) {
  assert [head, ..] = trace
  case instruction {
    "noop" -> [Trace(head.x, head.cycles + 1), ..trace]
    "addx " <> amount -> [
      Trace(head.x + parse_int(amount), head.cycles + 2),
      ..trace
    ]
  }
}

fn parse_int(text: String) -> Int {
  assert Ok(i) = int.parse(text)
  i
}

fn collect(
  trace: List(#(Trace, Trace)),
  want: List(Int),
  out: List(Int),
) -> List(Int) {
  case want {
    [] -> out
    [n, ..rest] ->
      case trace {
        [] -> {
          io.debug(out)
          abort("ran out of traces, want " <> int.to_string(n))
          []
        }
        [#(a, b), ..traces] ->
          case matches(n, a, b) {
            Some(x) -> collect(traces, rest, [x, ..out])
            None -> collect(traces, want, out)
          }
      }
  }
}

fn abort(msg: String) -> Nil {
  io.println(msg)
  process.kill(process.self())
}

fn matches(want: Int, a: Trace, b: Trace) -> Option(Int) {
  case want >= a.cycles && want <= b.cycles {
    True -> Some(a.x)
    False -> None
  }
}

fn signal_strength(pair: #(Int, Int)) -> Int {
  pair.0 * pair.1
}

fn inflate(out: List(Trace), pair: #(Trace, Trace)) -> List(Trace) {
  let #(prev, this) = pair
  let inflated =
    list.range(prev.cycles, this.cycles - 1)
    |> list.map(fn(i) { Trace(x: prev.x, cycles: i) })
  list.append(out, inflated)
}

fn render(display: List(String), trace: Trace) -> List(String) {
  let x = trace.x
  case
    list.range(x - 1, x + 1)
    |> list.contains(trace.cycles % display_width)
  {
    True -> ["#", ..display]
    False -> [" ", ..display]
  }
}
