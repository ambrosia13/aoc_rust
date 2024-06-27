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

    let mut converted = String::new();

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut i = 0;

        'character_iter: while i < bytes.len() {
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

            converted.push(bytes[i] as char);

            i += 1;
        }

        // let mut replaced_word_length = 0;
        //
        // for (index, ch) in line.trim().char_indices() {
        //     'replace_word: for &(word, digit) in NUMBERS {
        //         if line
        //             .get(index..(index + word.len()))
        //             .is_some_and(|lookahead| lookahead == word)
        //         {
        //             converted.push(char::from_digit(digit, 10).unwrap());
        //             replaced_word_length = word.len();
        //
        //             break 'replace_word;
        //         }
        //     }
        //
        //     if replaced_word_length == 0 {
        //         converted.push(ch);
        //     } else {
        //         replaced_word_length -= 1;
        //     }
        // }

        converted.push('\n');
    }

    converted
}

pub fn part_one() -> u32 {
    sum_calibrations(include_str!("input.txt"))
}

pub fn part_two() -> u32 {
    let input = convert_words_to_digits(include_str!("test_input.txt"));
    sum_calibrations(&input)
}
