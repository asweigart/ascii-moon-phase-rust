use clap::{ArgGroup, Parser, ValueEnum};
use time::Date;

use ascii_moon_phase::{parse_date_ymd, render_moon, render_moon_with_phase, moon_phase};

#[derive(Copy, Clone, ValueEnum)]
enum Hemisphere {
    North,
    South,
}

#[derive(Parser)]
#[command(
    name = "ascii-moon",
    about = "Render the lunar phase as filled ASCII art.",
    version
)]
#[command(group(ArgGroup::new("chars").multiple(true)))]
struct Args {
    /// Height in rows (width is 2*size)
    #[arg(long, default_value_t = 24)]
    size: usize,

    /// Orientation (north: waxing on RIGHT; south: mirrored)
    #[arg(long, value_enum, default_value_t = Hemisphere::North)]
    hemisphere: Hemisphere,

    /// Date to render (YYYY-MM-DD). Default: today (UTC).
    #[arg(long)]
    date: Option<String>,

    /// Phase to render (overrides --date). 0.0=new, 0.5=full, 1.0=new.
    #[arg(long)]
    phase: Option<f64>,

    /// Character for illuminated area
    #[arg(long, default_value = "@")]
    light_char: String,

    /// Character for dark area
    #[arg(long, default_value = ".")]
    dark_char: String,

    /// Character outside the disc
    #[arg(long, default_value = " ")]
    empty_char: String,

    /// Print the numeric phase after the art
    #[arg(long)]
    show_phase: bool,
}

fn first_char(s: &str, default: char) -> char {
    s.chars().next().unwrap_or(default)
}

fn main() {
    let args = Args::parse();

    // Parse date if provided
    let date: Option<Date> = match &args.date {
        None => None,
        Some(s) => Some(parse_date_ymd(s).unwrap_or_else(|e| {
            eprintln!("Invalid --date: {e}");
            std::process::exit(2);
        })),
    };

    // Validate phase if provided
    if let Some(p) = args.phase {
        if !(0.0..=1.0).contains(&p) {
            eprintln!("Invalid --phase: must be between 0.0 and 1.0");
            std::process::exit(2);
        }
    }

    let northern = matches!(args.hemisphere, Hemisphere::North);
    let light = first_char(&args.light_char, '@');
    let dark = first_char(&args.dark_char, '.');
    let empty = first_char(&args.empty_char, ' ');

    // --phase overrides --date
    let moon = if let Some(p) = args.phase {
        render_moon_with_phase(args.size, northern, p, light, dark, empty)
    } else {
        render_moon(args.size, northern, date, light, dark, empty)
    };

    println!("{moon}");

    if args.show_phase {
        let p = match args.phase {
            Some(p) => p,
            None => moon_phase(date),
        };
        let status = if (p - 0.5).abs() < 1e-12 {
            "full"
        } else if p < 0.5 {
            "waxing"
        } else {
            "waning"
        };
        println!("\nphase={:.6}  ({})", p, status);
    }
}
