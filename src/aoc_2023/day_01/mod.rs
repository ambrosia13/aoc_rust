fn input() -> &'static str {
    include_str!("input.txt")
}

fn collect_digits_in_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|ch| ch.to_digit(10)).collect()
}

fn sum_calibrations(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(collect_digits_in_line)
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

fn convert_words_to_digits(input: &str) -> String {
    const NUMBERS: &[(&str, char)] = &[
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    const MIN_NUMBER_WORD_LEN: usize = 2;

    let mut converted = String::new();

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut i = 0;

        // We only need to iterate until the length minus the length of the shortest number-word
        let iter_bound = line.len();
        let iter_bound = iter_bound
            .checked_sub(MIN_NUMBER_WORD_LEN)
            .unwrap_or(iter_bound);

        'character_iter: while i < iter_bound {
            for &(word, digit) in NUMBERS {
                if line
                    .get(i..(i + word.len()))
                    .is_some_and(|lookahead| lookahead == word)
                {
                    converted.push(digit);

                    // We found a word to replace, advance the pointer by the word length
                    // and continue searching from there
                    i += word.len();
                    continue 'character_iter;
                }
            }

            // No replacement was found, just add the char as-is
            converted.push(bytes[i] as char);

            i += 1;
        }

        // Push the part of the string we didn't iterate over, since a word can't be replaced
        // in that substring
        converted.push_str(&line[iter_bound..line.len()]);

        converted.push('\n');
    }

    converted
}

pub fn part_one() -> u32 {
    sum_calibrations(include_str!("input.txt"))
}

pub fn part_two() -> u32 {
    let input = convert_words_to_digits(include_str!("input.txt"));
    sum_calibrations(&input)
}
