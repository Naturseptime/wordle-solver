# worlde-solver
Wordle solver using optimized words for guessing. Written in Rust.

## Setup 
Download a english word list (e.g. https://github.com/dwyl/english-words) and put it into a file words.txt

## Running

**Input:**
1) The word you have guessed (e.g. PIZZA)
2) The hints for this word (e.g. ?1000).

Syntax for hints: 
_0 = letter not in word_,
_1 = correct letter here_,
_? = correct letter elsewhere_

**Output:**

A list with good candinates for guessing.

There are two kinds of word candinates.
* Words for quickly reducing the search space.
* Words for finally solving the riddle.

For each guessed word, the program also outputs the possible worst hint. 
So one could play a two-player-variant of Wordle where one player want to solve the riddle quickly and another player tries to slow him down by giving bad hints.