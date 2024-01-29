use termion::color;

pub fn print_giant_cat() {
    println!("{}   /\\_/\\", color::Fg(color::LightMagenta));
    println!("{}  ( o.o )", color::Fg(color::LightMagenta));
    println!(
        "{}   > ^ <{}",
        color::Fg(color::LightMagenta),
        color::Fg(color::Reset)
    );
}
