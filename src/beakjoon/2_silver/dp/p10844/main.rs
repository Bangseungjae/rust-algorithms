use std::io::stdin;

const MOD: u64 = 1_000_000_000;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut d: Vec<Vec<u64>> = vec![vec![0; 10]; n + 1];

    for i in 1..10 {
        d[1][i] = 1;
    }

    for i in 2..=n {
        d[i][0] = d[i - 1][1];
        d[i][9] = d[i - 1][8];

        for j in 1..9 {
            d[i][j] = (d[i - 1][j - 1] + d[i - 1][j + 1]) % MOD;
        }
    }

    let sum = d[n].iter().sum::<u64>() % MOD;
    let a: i16;

    println!("{sum}");
}
