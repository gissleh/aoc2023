use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct BothParts<T, U>(pub T, pub U);

impl<T, U> Display for BothParts<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

pub struct ResultCarrying<T, U>(pub T, pub U);

impl<T, U> ResultCarrying<T, U> {
    pub fn res(self) -> T {
        self.0
    }

    pub fn carry(self) -> U {
        self.1
    }
}

impl<T, U> Display for ResultCarrying<T, U>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (+ carrying data)", self.0)
    }
}

pub fn format_duration(ns: i64) -> String {
    if ns == i64::MAX {
        return "-".to_string();
    }

    if ns > 10_000_000_000 {
        format!("{:.1}s", (ns as f64) / (1_000_000_000 as f64))
    } else if ns > 1_000_000_000 {
        format!("{:.2}s", (ns as f64) / (1_000_000_000 as f64))
    } else if ns > 1_000_000 {
        format!("{:.2}ms", (ns as f64) / (1_000_000 as f64))
    } else if ns > 1_000 {
        format!("{:.2}µs", (ns as f64) / (1_000 as f64))
    } else {
        format!("{}ns", ns)
    }
}
