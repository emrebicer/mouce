#[cfg(feature = "cli")]
use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new("mouce")
        .about("A CLI tool that simulates mouse actions using the mouce library")
        .author("Emre Bicer")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("move_to")
                .about("Moves the mouse to the given position")
                .arg(Arg::new("x_position")
                    .long("x_position")
                    .short('x')
                .takes_value(true))
                .arg(Arg::new("y_position")
                    .long("y_position")
                    .short('y')
                .takes_value(true))
        )
        .subcommand(
            Command::new("get_position")
                .about("Get the current position of the mouse, outputs `x` and `y` coordinates seperated with a space")
        )
        .subcommand(
            Command::new("press_button")
                .about("Press the given mouse button")
                .arg(Arg::new("button")
                    .long("button")
                    .short('b')
                .takes_value(true))
        )
        .subcommand(
            Command::new("release_button")
                .about("Release the given mouse button")
                .arg(Arg::new("button")
                    .long("button")
                    .short('b')
                .takes_value(true))
        )
        .subcommand(
            Command::new("click_button")
                .about("Click the given mouse button")
                .arg(Arg::new("button")
                    .long("button")
                    .short('b')
                .takes_value(true))
        )
        .subcommand(
            Command::new("scroll_wheel")
                .about("Scroll the mouse wheel towards to given direction")
                .arg(Arg::new("direction")
                    .long("direction")
                    .short('d')
                .takes_value(true))
        );

    let mouse_manager = mouce::Mouse::new();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("move_to", sub_matches)) => {
            let x: usize = sub_matches.value_of_t_or_exit("x_position");
            let y: usize = sub_matches.value_of_t_or_exit("y_position");
            mouse_manager.move_to(x, y);
        }
        Some(("get_position", _)) => {
            let (x, y) = mouse_manager.get_position();
            println!("{x} {y}");
        }
        Some(("press_button", sub_matches)) => {
            let button_arg: String = sub_matches.value_of_t_or_exit("button");
            let button = get_mouse_button(&button_arg)?;
            mouse_manager.press_button(&button);
        }
        Some(("release_button", sub_matches)) => {
            let button_arg: String = sub_matches.value_of_t_or_exit("button");
            let button = get_mouse_button(&button_arg)?;
            mouse_manager.release_button(&button);
        }
        Some(("click_button", sub_matches)) => {
            let button_arg: String = sub_matches.value_of_t_or_exit("button");
            let button = get_mouse_button(&button_arg)?;
            mouse_manager.click_button(&button);
        }
        Some(("scroll_wheel", sub_matches)) => {
            let direction_arg: String = sub_matches.value_of_t_or_exit("direction");
            let direction = get_scroll_direction(&direction_arg)?;
            mouse_manager.scroll_wheel(&direction);
        }
        _ => {
            panic!("unknown subcommand, please see mouce --help");
        }
    }

    Ok(())
}

fn get_mouse_button(
    button: &str,
) -> Result<mouce::common::MouseButton, Box<dyn std::error::Error>> {
    match button {
        "left" => Ok(mouce::common::MouseButton::Left),
        "right" => Ok(mouce::common::MouseButton::Right),
        "middle" => Ok(mouce::common::MouseButton::Middle),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "{} is not accepted as a button, please use left, right or middle",
                button
            ),
        ))),
    }
}

fn get_scroll_direction(
    direction: &str,
) -> Result<mouce::common::ScrollDirection, Box<dyn std::error::Error>> {
    match direction {
        "up" => Ok(mouce::common::ScrollDirection::Up),
        "down" => Ok(mouce::common::ScrollDirection::Down),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "{} is not accepted as a direction, please use up or down",
                direction
            ),
        ))),
    }
}
