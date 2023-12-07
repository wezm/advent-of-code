#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <map>

using std::cout, std::cerr, std::endl;

// From strongest to weakest
enum class Type {
  // Five of a kind, where all five cards have the same label: AAAAA
  FiveOfAKind = 7,
  // Four of a kind, where four cards have the same label and one card has a
  // different label: AA8AA
  FourOfAKind = 6,
  // Full house, where three cards have the same label, and the remaining two
  // cards share a different label: 23332
  FullHouse = 5,
  // Three of a kind, where three cards have the same label, and the remaining
  // two cards are each different from any other card in the hand: TTT98
  ThreeOfAKind = 4,
  // Two pair, where two cards share one label, two other cards share a second
  // label, and the remaining card has a third label: 23432
  TwoPair = 3,
  // One pair, where two cards share one label, and the other three cards have a
  // different label from the pair and each other: A23A4
  OnePair = 2,
  // High card, where all cards' labels are distinct: 23456
  HighCard = 1,
  Nothing = 0,
};

class Hand {
  Hand(std::string, int);

  Type type();

  std::string hand;
  int bid;
};

Hand::Hand(std::string s, int n) : hand{s}, bid{n} {
  if (hand.length() != 5) {
    throw "hand length != 5";
  }
}

Type Hand::type()  {
  std::unordered_map<char, int> counts;

  for(auto c : hand) {
    counts[c] += 1;
  }

  if (counts.size() == 1) {
    return Type::FiveOfAKind;
  }
  else if (counts.size() == 2) {
    const auto it{counts.begin()};
    auto count{it->second};
    switch (count) {
      case 1:
      case 4:
        return Type::FourOfAKind;
      case 2:
      case 3:
        return Type::FullHouse;
    }

    for (const auto p : counts) {

    }
  }

  return Type::Nothing;
}

int main(int argc, char **argv) {
  if (argc < 2) {
    cerr << "Usage: day7 input.txt" << endl;
    return 1;
  }

  std::ifstream ifs{argv[1]};
  std::string input{(std::istreambuf_iterator<char>(ifs)),
                    (std::istreambuf_iterator<char>())};
  std::istringstream iss{input};

  for (std::string line; std::getline(iss, line);) {
    if (line.empty()) {
      continue;
    }

    std::istringstream liness{line};
    std::string hand;
    int bid;

    liness >> hand >> bid;
    cout << hand << ": " << bid << endl;


  }

  return 0;
}
