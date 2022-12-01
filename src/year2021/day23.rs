enum Pod {
    A,
    B,
    C,
    D,
}

fn energy(p: Pod) -> usize {
    match p {
        A => 1,
        B => 10,
        C => 100,
        D => 1000,
    }
}

pub fn part1(input: String) -> usize {
    0
}
