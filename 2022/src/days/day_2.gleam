import gleam/int
import gleam/list
import gleam/string

type Hand {
  Rock
  Paper
  Scissors
}

type Outcome {
  Lose
  Draw
  Win
}

type Game =
  #(Hand, Hand)

type RiggedGame =
  #(Hand, Outcome)

pub fn pt_1(input: String) -> Int {
  string.split(input, on: "\n")
  |> list.map(parse_line)
  |> list.map(score_game)
  |> int.sum
}

pub fn pt_2(input: String) -> Int {
  string.split(input, on: "\n")
  |> list.map(parse_line_pt2)
  |> list.map(generate_hand)
  |> list.map(score_game)
  |> int.sum
}

fn parse_line(line: String) -> Game {
  assert [a, b] = string.split(line, on: " ")
  #(to_hand(a), to_hand(b))
}

fn parse_line_pt2(line: String) -> RiggedGame {
  assert [a, b] = string.split(line, on: " ")
  #(to_hand(a), to_outcome(b))
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

// X means you need to lose,
// Y means you need to end the round in a draw,
// and Z means you need to win.
fn to_outcome(raw: String) -> Outcome {
  case raw {
    "X" -> Lose
    "Y" -> Draw
    "Z" -> Win
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

/// Determine the hand to play give the opponent's hand and desired outcome
fn generate_hand(game: RiggedGame) -> Game {
  case game {
    #(Rock, Lose) -> #(Rock, Scissors)
    #(Rock, Win) -> #(Rock, Paper)
    #(Paper, Lose) -> #(Paper, Rock)
    #(Paper, Win) -> #(Paper, Scissors)
    #(Scissors, Lose) -> #(Scissors, Paper)
    #(Scissors, Win) -> #(Scissors, Rock)
    #(_, Draw) -> #(game.0, game.0)
  }
}
