use clap::{arg, command, ArgAction};

fn letter_case_permutation(str: String) -> Vec<String> {
    let mut result = Vec::new();
    result.push(String::from(""));
    for char in str.chars().into_iter() {
        let n = result.len();
        if char.is_alphabetic() {
            for i in 0..n {
                result.push(result[i].clone());
                result[i].push(char.to_ascii_uppercase());
                result[n + i].push(char.to_ascii_lowercase());
            }
        } else {
            for i in 0..n {
                result[i].push(char);
            }
        }
    }
    result
}

fn main() {
    let matches = command!()
        .arg(arg!([name] "the name of env variable"))
        .arg(
            arg!(
                -i --ignore "ignore case of env variable"
            )
            .action(ArgAction::SetTrue),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").unwrap();

    if *matches.get_one::<bool>("ignore").expect("default false") {
        let mut result = letter_case_permutation(name.to_string());
        loop {
            if result.is_empty() {
                break;
            }
            if let Ok(value) = std::env::var(result.pop().unwrap_or_else(|| "".to_string())) {
                println!("{}", value);
                break;
            }
        }
    } else {
        println!("{}", std::env::var(name).unwrap_or_default());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_case_permutation() {
        let mut result = letter_case_permutation(String::from("a1b2"));
        let mut result1 = vec![
            String::from("a1b2"),
            String::from("a1B2"),
            String::from("A1b2"),
            String::from("A1B2"),
        ];
        result.sort();
        result1.sort();
        assert_eq!(result, result1);
    }
}
