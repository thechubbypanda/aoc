use std::fmt::{Display, Formatter};

pub enum InputType {
    Test,
    Real,
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InputType::Test => "test",
            InputType::Real => "real",
        })
    }
}
