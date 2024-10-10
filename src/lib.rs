use std::{error::Error, io};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut sentence = String::new();

    print!("English: ");

    io::Write::flush(&mut io::stdout())?;
    io::stdin().read_line(&mut sentence)?;

    let result = translate(&sentence);

    println!("{result}");

    Ok(())
}

fn translate(s: &str) -> String {
    let mut result = String::new();

    s.split_whitespace().for_each(|mut word| {
        let mut consonants = String::new();
        let mut counter: u32 = 0;

        loop {
            if starts_with_vowel(word) {
                if counter == 0 {
                    result = format!("{result} {word}way");
                } else {
                    result = format!("{result} {word}{consonants}ay")
                }

                break;
            } else {
                let value = word.chars().next().unwrap().to_string();
                consonants += &value;
                let mut x = word.chars();
                x.next();
                word = x.as_str();
            }

            counter += 1;
        }
    });

    result.trim().to_ascii_lowercase().to_string()
}

fn starts_with_vowel(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        matches!(first_char.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_returns_correct() {
        let input = "this is pig latin";
        let expected = "isthay isway igpay atinlay";

        let actual = translate(input);

        assert_eq!(expected.to_string(), actual.to_string());
    }

    #[test]
    fn starts_with_vowel_returns_true_if_true() {
        let input = "apple";
        let expected = true;

        let actual = starts_with_vowel(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn starts_with_vowel_returns_false_if_false() {
        let input = "banana";
        let expected = false;

        let actual = starts_with_vowel(input);

        assert_eq!(expected, actual);
    }
}