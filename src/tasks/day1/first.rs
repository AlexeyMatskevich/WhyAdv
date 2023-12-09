use std::fs;
use rayon::prelude::*;

pub fn first_try() -> i32 {
    let input = fs::read_to_string("src/tasks/day1/input").unwrap();
    let mut counter = 0;
    for line in input.lines() {
        let mut first_value = String::from("");
        let mut second_value = String::from("");

        for char in line.chars() {
            if char.is_numeric() {
                first_value = first_value + &char.to_string();
                break;
            }
        }

        for char in line.chars() {
            if char.is_numeric() {
                second_value = second_value + &char.to_string();
                break;
            }
        }

        let diff = format!("{}{}", first_value, second_value).parse().unwrap_or(0);
        counter = counter + diff;
    }

    return counter;
}

pub fn second_try() -> i32 {
    let input = fs::read_to_string("src/tasks/day1/input").unwrap();
    let mut counter = 0;
    for line in input.lines() {
        let mut result_chars  = Vec::new();

        for char in line.chars() {
            if char.is_numeric() {
                result_chars.insert(0, char);
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                result_chars.push(char);
                break;
            }
        }

        counter = counter + result_chars.into_iter().collect::<String>().parse().unwrap_or(0);
    }

    return counter;
}

fn get_number(line: &str) ->i32 {
    let mut result_chars  = Vec::new();

    for char in line.chars() {
        if char.is_numeric() {
            result_chars.insert(0, char);
            break;
        }
    }

    for char in line.chars().rev() {
        if char.is_numeric() {
            result_chars.push(char);
            break;
        }
    }

    return result_chars.into_iter().collect::<String>().parse().unwrap_or(0);
}

pub fn third_try() -> i32 {
    let input = fs::read_to_string("src/tasks/day1/input").unwrap();

    input
        .par_lines()
        .map(|l| get_number(l))
        .sum()
}

pub fn fourth_try() -> i32 {
    let input = fs::read_to_string("src/tasks/day1/input").unwrap();
    let mut counter = 0;
    for line in input.lines() {
        let mut result_chars = ['s'; 2];

        for char in line.chars() {
            if char.is_numeric() {
                result_chars[0] = char;
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                result_chars[1] = char;
                break;
            }
        }

        counter = counter + result_chars.into_iter().collect::<String>().parse().unwrap_or(0);
    }

    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let result = first_try();
        assert_eq!(result, 55477);
    }

    #[test]
    fn second_test() {
        let result = second_try();
        assert_eq!(result, 55477);
    }

    #[test]
    fn third_test() {
        let result = third_try();
        assert_eq!(result, 55477);
    }

    #[test]
    fn fourth_test() {
        let result = fourth_try();
        assert_eq!(result, 55477);
    }
}