import gleam/int
import gleam/list
import gleam/string

type Hand {
  Rock
  Paper
  Scissors
}

type Game =
  #(Hand, Hand)

pub fn pt_1(input: String) -> Int {
  string.split(input, on: "\n")
  |> list.map(parse_line)
  |> list.map(score_game)
  |> int.sum
}

pub fn pt_2(_input: String) -> Int {
  todo
}

fn parse_line(line: String) -> Game {
  assert [a, b] = string.split(line, on: " ")
  #(to_hand(a), to_hand(b))
}

fn to_hand(raw: String) -> Hand {
  case raw {
    "A" -> Rock
    "B" -> Paper
    "C" -> Scissors
    "X" -> Rock
    "Y" -> Paper
    "Z" -> Scissors
  }
}

fn score_game(game: Game) -> Int {
  let score = case game {
    // (them, me)
    #(Rock, Paper) -> 6
    #(Rock, Scissors) -> 0
    #(Paper, Rock) -> 0
    #(Paper, Scissors) -> 6
    #(Scissors, Rock) -> 6
    #(Scissors, Paper) -> 0
    #(Rock, Rock) -> 3
    #(Paper, Paper) -> 3
    #(Scissors, Scissors) -> 3
  }
  score + score_hand(game.1)
}

fn score_hand(hand: Hand) -> Int {
  case hand {
    Rock -> 1
    Paper -> 2
    Scissors -> 3
  }
}
