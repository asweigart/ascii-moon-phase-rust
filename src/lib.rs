use std::f64::consts::PI;
use time::{Date, Month, OffsetDateTime}; // ensure OffsetDateTime is imported

const SYNODIC_MONTH: f64 = 29.530_588_853; // days
const REF_JD: f64 = 2_451_550.1;          // near new moon

fn julian_day_noon_utc(date: Date) -> f64 {
    let mut y = date.year();
    let mut m = date.month() as i32;
    let d = date.day() as f64 + 12.0 / 24.0;

    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let a = (y as i32 / 100) as i32;
    let b = 2 - a + (a / 4);

    ((365.25 * (y as f64 + 4716.0)).floor())
        + ((30.6001 * ((m + 1) as f64)).floor())
        + d + (b as f64) - 1524.5
}

/// 0.0=new, 0.5=full, 1.0=new (again).
pub fn moon_phase(date: Option<Date>) -> f64 {
    let date = date.unwrap_or_else(|| OffsetDateTime::now_utc().date());
    let jd = julian_day_noon_utc(date);
    let mut p = ((jd - REF_JD) / SYNODIC_MONTH) % 1.0;
    if p < 0.0 {
        p += 1.0;
    }
    p
}

/// Render using a *date* (backwards compatible).
pub fn render_moon(
    size: usize,
    northern_hemisphere: bool,
    date: Option<Date>,
    light_char: char,
    dark_char: char,
    empty_char: char,
) -> String {
    let p = moon_phase(date);
    render_moon_with_phase(
        size,
        northern_hemisphere,
        p,
        light_char,
        dark_char,
        empty_char,
    )
}

/// Render using an explicit *phase* in [0.0, 1.0].
pub fn render_moon_with_phase(
    size: usize,
    northern_hemisphere: bool,
    phase: f64,
    light_char: char,
    dark_char: char,
    empty_char: char,
) -> String {
    assert!(size >= 2, "size must be at least 2");
    assert!(
        (0.0..=1.0).contains(&phase),
        "phase must be in [0.0, 1.0]"
    );

    let height = size;
    let width = size * 2;

    let theta = 2.0 * PI * phase; // 0=new, pi=full
    let sx = if northern_hemisphere { -theta.sin() } else { theta.sin() };
    let sz = theta.cos();

    let mut out = String::with_capacity((width + 1) * height);

    for j in 0..height {
        let y = 2.0 * ((j as f64 + 0.5) / height as f64) - 1.0;
        for i in 0..width {
            let x = 2.0 * ((i as f64 + 0.5) / width as f64) - 1.0;
            let r2 = x * x + y * y;
            if r2 <= 1.0 {
                let z2 = 1.0 - r2;
                let z = if z2 > 0.0 { z2.sqrt() } else { 0.0 };
                let lit = (sx * x + sz * z) < 0.0;
                out.push(if lit { light_char } else { dark_char });
            } else {
                out.push(empty_char);
            }
        }
        if j + 1 < height {
            out.push('\n');
        }
    }

    out
}

/// Convenience: parse YYYY-MM-DD to `Date`.
pub fn parse_date_ymd(s: &str) -> Result<Date, String> {
    let parts: Vec<_> = s.split('-').collect();
    if parts.len() != 3 {
        return Err("expected YYYY-MM-DD".into());
    }
    let y: i32 = parts[0].parse().map_err(|_| "invalid year")?;
    let m: u8 = parts[1].parse().map_err(|_| "invalid month")?;
    let d: u8 = parts[2].parse().map_err(|_| "invalid day")?;
    let month = Month::try_from(m).map_err(|_| "month out of range")?;
    Date::from_calendar_date(y, month, d).map_err(|_| "invalid date".into())
}
