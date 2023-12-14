use super::*;

pub struct WrapDisplay<'i, T> {
    inner: &'i T,
}

impl<'i, T> Debug for WrapDisplay<'i, T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.inner, f)
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Record")
            .field("expression", &WrapDisplay { inner: &self.e })
            .field("value", &WrapDisplay { inner: &self.n })
            .finish()
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atomic { number } => {
                Display::fmt(number, f)
            }
            Self::Negative { lhs } => {
                f.debug_struct("Negative")
                    .field("lhs", lhs)
                    .finish()
            }
            Self::Concat { lhs, rhs } => {
                f.debug_struct("Concat")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Plus { lhs, rhs } => {
                f.debug_struct("Plus")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Minus { reverse, lhs, rhs } => {
                f.debug_struct("Minus")
                    .field("reverse", reverse)
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Times { lhs, rhs } => {
                f.debug_struct("Times")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Divide { lhs, rhs } => {
                f.debug_struct("Divide")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atomic { number } => {
                Display::fmt(number, f)
            }
            Self::Negative { lhs } => {
                write!(f, "-{lhs}")
            }
            Self::Concat { lhs, rhs } => {
                f.debug_struct("Concat")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Plus { lhs, rhs } => {
                write!(f, "{lhs}+{rhs}")
            }
            Self::Minus { reverse, lhs, rhs } => {
                f.debug_struct("Minus")
                    .field("reverse", reverse)
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Times { lhs, rhs } => {
                f.debug_struct("Times")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
            Self::Divide { lhs, rhs } => {
                f.debug_struct("Divide")
                    .field("lhs", lhs)
                    .field("rhs", rhs)
                    .finish()
            }
        }
    }
}