use std::io::stdin;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

pub fn ask_until<T, E, P, C>(prompt: &str, parser: P, condition: C) -> T
where
    P: Fn(&str) -> Result<T, E>,
    C: Fn(&T) -> bool,
{
    loop {
        let to_check = match parser(&get_input(prompt)) {
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

pub fn handle_key_input(event: Event) {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {} |
        _ => {}
    }
}
