use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
use state::CalloopData;

mod backend;
mod handlers;
mod state;

#[derive(Debug)]
enum Backend {
    // Runs the compositor in a Wayland or X11 window
    Winit,

    // Runs the compositor in a TTY client
    TtyUdev,
}

#[derive(Debug)]
struct StartSettings {
    backend: Backend,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable tracing showing in cmd
    init_tracing();

    let args: Vec<String> = std::env::args().collect();

    // This will be changed later
    let mut start_settings = StartSettings {
        backend: Backend::Winit,
    };

    parse_args(args, &mut start_settings);

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;
    let display: Display<state::Waytest> = Display::new()?;
    let display_handle = display.handle();
    let state = state::Waytest::new(&mut event_loop, display);
    let mut data = CalloopData {
        state,
        display_handle,
    };

    match start_settings.backend {
        Backend::Winit => {
            tracing::info!("Starting compositor with winit backend.");
            backend::winit::run_winit(&mut event_loop, &mut data)?;
        }
        Backend::TtyUdev => {
            tracing::info!("Starting compositor with tty-udev backend.");
        }
    }

    event_loop.run(None, &mut data, move |_| {
        // Compositor is running
    })?;

    tracing::info!("Compositor is shutting down.");

    Ok(())
}

fn parse_args(args: Vec<String>, start_settings: &mut StartSettings) {
    let mut waiting_value: bool = false;
    let mut waiting_value_to: &str = "nothing";
    let mut index = 0;

    for arg in args {
        if index != 0 {
            if waiting_value {
                match waiting_value_to {
                    "backend" => match arg.as_str() {
                        "winit" => start_settings.backend = Backend::Winit,
                        "tty-udev" => start_settings.backend = Backend::TtyUdev,
                        _ => {
                            tracing::error!("Expected a valid backend: [\"winit\", \"tty-udev\"]");
                            std::process::exit(1);
                        }
                    },
                    _ => tracing::error!("{} not implemented yet.", arg),
                }

                waiting_value = false;
            } else {
                match arg.as_str() {
                    "--backend" => {
                        waiting_value = true;
                        waiting_value_to = "backend";
                    }
                    _ => {
                        tracing::error!("{}: unexpected argument.", arg);
                        std::process::exit(1);
                    }
                }
            }
        }

        index += 1;
    }

    // If we are expecting a value and the arguments come out
    if waiting_value {
        tracing::error!("Expected a value to --{}.", waiting_value_to);
        std::process::exit(1);
    }
}

fn init_tracing() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
        tracing::info!("Tracing started with env filter.");
    } else {
        tracing_subscriber::fmt().init();
        tracing::info!("Tracing started.");
    }
}
