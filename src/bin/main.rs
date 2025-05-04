use clap::{Parser, Subcommand};
use mouce::MouseActions;
use std::thread::sleep;
use std::time::Duration;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(
    name = "mouce",
    about = "A CLI tool that simulates mouse actions using the mouce library",
    author = "Emre Bicer",
    version = env!("CARGO_PKG_VERSION")
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Moves the mouse to the given position in logical pixel space
    MoveTo {
        #[arg(long, short)]
        x_position: i32,
        #[arg(long, short)]
        y_position: i32,
    },
    /// Move the mouse relative to the current position in logical pixel space
    MoveRelative {
        #[arg(long, short)]
        x_offset: i32,
        #[arg(long, short)]
        y_offset: i32,
    },
    /// Get the current position of the mouse in logical pixel space, outputs `x` and `y` coordinates separated with a space
    GetPosition,
    /// Press the given mouse button
    PressButton {
        #[arg(long, short)]
        button: String,
    },
    /// Release the given mouse button
    ReleaseButton {
        #[arg(long, short)]
        button: String,
    },
    /// Click the given mouse button
    ClickButton {
        #[arg(long, short)]
        button: String,
    },
    /// Scroll the mouse wheel towards the given direction
    ScrollWheel {
        #[arg(long, short)]
        direction: String,
        #[arg(long, short)]
        amount: u32,
    },
    /// Listen mouse events and print them to the terminal
    Listen,
    /// Listen for left mouse button press and print the position when pressed
    GetPositionOnClick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut mouse_manager = mouce::Mouse::new();

    match cli.command {
        Commands::MoveTo {
            x_position,
            y_position,
        } => {
            mouse_manager.move_to(x_position, y_position)?;
        }
        Commands::MoveRelative { x_offset, y_offset } => {
            mouse_manager.move_relative(x_offset, y_offset)?;
        }
        Commands::GetPosition => {
            let (x, y) = mouse_manager.get_position()?;
            println!("{x} {y}");
        }
        Commands::PressButton { button } => {
            let button = get_mouse_button(&button)?;
            mouse_manager.press_button(&button)?;
        }
        Commands::ReleaseButton { button } => {
            let button = get_mouse_button(&button)?;
            mouse_manager.release_button(&button)?;
        }
        Commands::ClickButton { button } => {
            let button = get_mouse_button(&button)?;
            mouse_manager.click_button(&button)?;
        }
        Commands::ScrollWheel { direction, amount } => {
            let direction = get_scroll_direction(&direction)?;
            mouse_manager.scroll_wheel(&direction, amount)?;
        }
        Commands::Listen => {
            mouse_manager.hook(Box::new(|event| {
                println!("{:?}", event);
            }))?;
            loop {
                sleep(Duration::from_secs(u64::max_value()));
            }
        }
        Commands::GetPositionOnClick => {
            let manager_clone = mouse_manager.clone();
            mouse_manager.hook(Box::new(move |e| {
                match e {
                    mouce::common::MouseEvent::Press(mouce::common::MouseButton::Left) => {
                        match manager_clone.get_position() {
                            Ok((x, y)) => println!("{x} {y}"),
                            Err(err) => println!("Failed to get current position: {err}"),
                        }
                    }
                    _ => {}
                };
            }))?;
            loop {
                sleep(Duration::from_secs(u64::max_value()));
            }
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
        "right" => Ok(mouce::common::ScrollDirection::Right),
        "left" => Ok(mouce::common::ScrollDirection::Left),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "{} is not accepted as a direction, please use up, down, right or left",
                direction
            ),
        ))),
    }
}
