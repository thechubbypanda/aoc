use std::str::FromStr;

pub fn to_lines(input: &String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn parse_strings<T>(input: &[String]) -> Result<Vec<T>, T::Err>
where
    T: FromStr,
{
    input.iter().map(|s| s.parse()).collect()
}

pub fn transpose<T>(input: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(input.windows(2).all(|w| w[0].len() == w[1].len()));
    (0..input[0].len())
        .map(|i| input.iter().map(|line| line[i]).collect())
        .collect()
}
