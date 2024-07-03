fn collect_digits_in_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|ch| ch.to_digit(10)).collect()
}

fn collect_numbers_in_line(line: &str) -> Vec<u32> {
    const NUMBERS: &[(&str, u32)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let bytes = line.as_bytes();

    let mut digits = Vec::new();

    let mut i = 0;

    'character_iter: while i < line.len() {
        let ch = bytes[i] as char;

        if ch.is_ascii_digit() {
            digits.push(ch.to_digit(10).unwrap());
        } else {
            for &(word, digit) in NUMBERS {
                if line
                    .get(i..(i + word.len()))
                    .is_some_and(|lookahead| lookahead == word)
                {
                    digits.push(digit);

                    // We found a word to replace, advance the pointer by the word length
                    // and continue searching from there
                    if [4, 6, 8].contains(&digit) {
                        i += word.len();
                    } else {
                        // Since these words can chain other numbers, e.g. "twone",
                        // advance one less character
                        i += word.len() - 1;
                    }
                    continue 'character_iter;
                }
            }
        }

        i += 1;
    }

    unsafe {
        std::arch::asm! {
            "nop"
        }
    }
    digits
}

fn sum_calibrations<F>(input: &str, collect_digits: F) -> u32
where
    F: FnMut(&str) -> Vec<u32>,
{
    input
        .trim()
        .lines()
        .map(collect_digits)
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

pub fn part_one() -> u32 {
    sum_calibrations(include_str!("input.txt"), collect_digits_in_line)
}

pub fn part_two() -> u32 {
    sum_calibrations(include_str!("input.txt"), collect_numbers_in_line)
}
