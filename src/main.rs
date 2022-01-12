use std::fs::File;
use std::io::{self, BufRead};

type CharNumber = u8;
type BitMask = u32;

const L: usize = 26; // Number of chars (2^L-1 should fit BitMask type)
const N: usize = 5; // Number of slots (3^N-1 should fit in usize type)

#[derive(Debug)]
struct Candidates {
    masks: [BitMask; N],
    used: BitMask
}

impl Candidates {
    fn new() -> Self {
        Self {
            masks: [(1 << L) - 1; N],
            used: 0
        }
    }

    fn word_fits(&self, word: &[CharNumber]) -> bool {
        let accum = word.iter().fold(0, |x, y| x | (1 << y));
        self.masks
            .iter()
            .zip(word.iter())
            .all(|(mask, ch_num)| mask & (1 << ch_num) != 0)
            && (self.used & accum == self.used)
    }

    fn apply_hints(&mut self, guessed: &[CharNumber], hints: &[Hint]) {
        for (i, (letter, hint)) in guessed.iter().zip(hints.iter()).enumerate() {
            match hint {
                Hint::Here => self.masks[i] = 1 << letter,
                Hint::Elsewhere => {
                    self.masks[i] &= !(1 << letter);
                    self.used |= 1 << letter;
                }
                Hint::Nowhere => {
                    for m in &mut self.masks {
                        *m &= !(1 << letter)
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Hint {
    Here = 0,
    Elsewhere = 1,
    Nowhere = 2
}

type EncodedHint = usize;

fn encode_hints(hints: &[Hint]) -> EncodedHint {
    let mut a: EncodedHint = 0;
    let mut b = 1;
    for h in hints {
        a += (*h as EncodedHint) * b;
        b *= 3;
    }
    a
}

fn decode_hints(v: EncodedHint) -> Vec<Hint> {
    let mut result = Vec::new();
    let mut w = v;
    for _i in 0..N {
        result.push(match w % 3 {
            0 => Hint::Here,
            1 => Hint::Elsewhere,
            2 => Hint::Nowhere,
            _ => unreachable!()
        });
        w /= 3;
    }
    result
}

fn hints_from_string(input: &str) -> Result<Vec<Hint>, String> {
    if input.len() != N {
        return Err(format!("Hints must be {} character(s) long", N));
    }
    input
        .chars()
        .map(|ch| match ch {
            '1' => Ok(Hint::Here),
            '?' => Ok(Hint::Elsewhere),
            '0' => Ok(Hint::Nowhere),
            _ => Err(format!(
                "Invalid character {}, only '0', '1' and '?' allowed",
                ch
            ))
        })
        .collect()
}

fn compare_words(input: &[CharNumber], hidden: &[CharNumber]) -> Vec<Hint> {
    let mut result = vec![Hint::Nowhere; N];
    for i in 0..N {
        for j in 0..N {
            if input[i] == hidden[j] && i != j {
                result[i] = Hint::Elsewhere;
                break;
            }
        }

        if input[i] == hidden[i] {
            result[i] = Hint::Here
        }
    }
    result
}

fn char_number(c: char) -> Result<CharNumber, String> {
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_uppercase() as u8 - 65)
    } else {
        Err(format!("Invalid character: {}", c))
    }
}

fn string_numbers(s: &str) -> Result<Vec<CharNumber>, String> {
    if s.len() != N {
        Err(format!("Guessed word be {} character(s) long", N))
    } else {
        s.chars().map(|ch| char_number(ch)).collect()
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim().to_string()
}

fn find_word_score(current_solutions: &Vec<String>, cword: &String) -> (String, i64) {
    let mut results = vec![0_usize; 3_u32.pow(N as u32) as usize];
    for hword in current_solutions.iter() {
        results[encode_hints(&compare_words(
            &string_numbers(cword).unwrap(),
            &string_numbers(hword).unwrap()
        ))] += 1;
    }

    //~ let avg : f64 = results.iter().map(|x| *x as f64).sum::<f64>() / (results.len() as f64);
    //~ let dev : f64 = results.iter().map(|x| (*x as f64 - avg) * (*x as f64 - avg)).sum::<f64>().sqrt();
    //~ (cword.clone(), (dev * 100.0) as i64)

    let r = *results.iter().max().unwrap_or(&0);
    (cword.clone(), r as i64)
}

fn worst_hints(current_solutions: &Vec<String>, cword: &Vec<CharNumber>) -> Vec<Hint> {
    let mut results = vec![0_usize; 3_u32.pow(N as u32) as usize];
    for hword in current_solutions.iter() {
        results[encode_hints(&compare_words(cword, &string_numbers(hword).unwrap()))] += 1;
    }

    decode_hints(
        results
            .iter()
            .enumerate()
            .max_by_key(|(_, x)| x.clone())
            .unwrap()
            .0
    )
}

fn read_lines(filename: &str) -> Vec<String> {
    let reader =
        io::BufReader::new(File::open(filename).expect(&format!("File {} not found!", filename)));
    let lines = reader
        .lines()
        .into_iter()
        .collect::<Result<Vec<String>, _>>()
        .expect("Error readling lines");
    lines
        .into_iter()
        .filter(|w| w.len() == N && w.chars().all(|w| w.is_ascii_alphabetic()))
        .map(|w| w.to_uppercase())
        .collect()
}

fn main() {
    let all_solutions = read_lines("all_solutions.txt");
    let all_guesses = read_lines("all_guesses.txt");

    let mut current_solutions = all_solutions.clone();
    let mut ca = Candidates::new();

    loop {
        if current_solutions.is_empty() {
            println!("No words found.");
            break;
        }

        if current_solutions.len() == 1 {
            println!("Finally found word {}", current_solutions.first().unwrap());
            break;
        }

        let guess = loop {
            println!("Next Word for guessing:");
            let guess = string_numbers(&read_line());
            match guess {
                Ok(g) => break (g),
                Err(e) => println!("{}", e)
            }
        };

        println!("Worst hint: {:?}", worst_hints(&current_solutions, &guess));

        let hints = loop {
            println!("Hints for this word: (1 = here, 0 = nowhere, ? = elsewhere)");
            let hints = hints_from_string(&read_line());
            match hints {
                Ok(h) => break (h),
                Err(e) => println!("{}", e)
            }
        };

        println!("Computing words...\n");

        ca.apply_hints(&guess, &hints);

        current_solutions = current_solutions
            .iter()
            .filter(|word| ca.word_fits(&string_numbers(word).unwrap()))
            .cloned()
            .collect::<Vec<String>>();

        let mut best_guesses = all_guesses
            .iter()
            .map(|cword| find_word_score(&current_solutions, cword))
            .collect::<Vec<(String, i64)>>();
        best_guesses.sort_by_key(|(_, s)| s.clone());

        println!(
            "Guesses for quickly reducing the search space:\n{} \n",
            best_guesses
                .iter()
                .take(100)
                .map(|(x, y)| format!("{} ({})", x, y))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut final_words = current_solutions
            .iter()
            .map(|cword| find_word_score(&current_solutions, cword))
            .collect::<Vec<(String, i64)>>();
        final_words.sort_by_key(|(_, s)| s.clone());

        println!(
            "Final guesses:\n{} \n",
            final_words
                .iter()
                .take(20)
                .map(|(x, y)| format!("{} ({})", x, y))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}
