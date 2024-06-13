use std::{env::args, process::Command};

#[derive(Debug)]
enum Action {
    In,
    Out,
    Reset,
}

impl Action {
    fn from(action: Option<String>) -> Self {
        match action {
            Some(action) if &action == "out" => Self::Out,
            Some(action) if &action == "in" => Self::In,
            _ => Self::Reset,
        }
    }
}

fn main() {
    let action = Action::from(args().last());
    let step = get_step(action);
    zoom(step);
}

fn current_zoom_factor() -> f32 {
    // hyprctl getoption cursor:zoom_factor | lines | first | split row ':' | get 1 | str trim | into float
    let bytes = Command::new("hyprctl")
        .args(["getoption", "cursor:zoom_factor"])
        .output()
        .unwrap()
        .stdout;
    let binding = String::from_utf8(bytes).unwrap();
    binding
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .parse::<f32>()
        .unwrap()
}

const STEPS: [f32; 7] = [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

fn zoom(factor: f32) {
    let factor = format!("{factor:.2}");
    Command::new("hyprctl")
        .args(["keyword", "cursor:zoom_factor", factor.as_str()])
        .output()
        .unwrap();
}

fn get_step_index(czf: f32) -> usize {
    (0..STEPS.len()).find(|x| STEPS[*x] == czf).unwrap()
}

fn get_step(action: Action) -> f32 {
    let czf = current_zoom_factor();
    let index = get_step_index(czf);
    let len = STEPS.len();
    match action {
        Action::In if index < len - 1 => STEPS[index + 1],
        Action::In => STEPS.last().unwrap().clone(),
        Action::Out if index > 0 => STEPS[index - 1],
        Action::Out | Action::Reset => STEPS.first().unwrap().clone(),
    }
}
