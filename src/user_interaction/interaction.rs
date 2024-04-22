use std::io::stdin;

pub fn ask_until<T, E, P, C>(prompt: &str, parser: P, condition: C) -> T
where
    P: Fn(&str) -> Result<T, E>,
    C: Fn(&T) -> bool,
{
    loop {
        let val = get_input(prompt);
        let formated = parser(&val);

        let to_check = match formated {
            Ok(check) => check,
            Err(_) => continue,
        };

        if condition(&to_check) {
            break to_check;
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Failed to read line");

    answer
}
