use crate::input_type::InputType;
use std::path::{Path, PathBuf};

pub(crate) const INPUT_DIR: &str = "input";

pub fn get_day_path(year: u16, day: u8, input_type: InputType) -> PathBuf {
    let day = add_leading_zeros(day.to_string(), 2);
    Path::new(INPUT_DIR)
        .to_path_buf()
        .join(format!("year{}_day{}.{input_type}", year, day))
}

fn add_leading_zeros(n: String, desired_len: usize) -> String {
    let zeros = vec!['0'; desired_len - n.len()]
        .iter()
        .cloned()
        .collect::<String>();
    format!("{zeros}{n}")
}

#[cfg(test)]
mod tests {
    use crate::input_path::add_leading_zeros;

    #[test]
    fn test_add_leading_zeros() {
        assert_eq!(add_leading_zeros("1".to_string(), 2), "01");
    }
}
