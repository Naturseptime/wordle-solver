# worlde-solver
Wordle solver using optimized words for guessing. Written in Rust.

## Running

    cargo run --release

**Input:**
1) The word you have guessed (e.g. PIZZA)
2) The hints for this word (e.g. ?1000).

Syntax for hints: 
_0 = letter not in word_,
_1 = correct letter here_,
_? = correct letter elsewhere_

The program needs two files all_solutions.txt (for final solutions) and all_guesses.txt (for intermediate guesses).

**Output:**

A list with good candinates for guessing.

There are two kinds of word candinates.
* Words for quickly reducing the search space.
* Words for finally solving the riddle.

For each guessed word, the program also outputs the possible worst hint. 
So one could play a two-player-variant of Wordle where one player want to solve the riddle quickly and another player tries to slow him down by giving bad hints.

## Solution methods

On each step the program searches for the best word for this step under following condition:

If we split the solutions into categories corresponding to the different 3⁵ possible hints, the largest of these categories should be minimal size.