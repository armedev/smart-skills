use colored::Colorize;

pub struct Colors;

impl Colors {
    pub fn header(s: &str) -> colored::ColoredString {
        s.blue()
    }

    pub fn success(s: &str) -> colored::ColoredString {
        s.green()
    }

    pub fn error(s: &str) -> colored::ColoredString {
        s.red()
    }

    pub fn warning(s: &str) -> colored::ColoredString {
        s.yellow()
    }

    pub fn skill(s: &str) -> colored::ColoredString {
        s.magenta()
    }

    pub fn dim(s: &str) -> colored::ColoredString {
        s.dimmed()
    }
}
