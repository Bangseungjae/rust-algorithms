use std::io::stdin;

fn main() {
    let mut str = String::new();
    stdin().read_line(&mut str).expect("TODO: panic message");
    let n: usize = str.trim().parse().unwrap();
    let mut arr: Vec<Vec<u64>> = vec![vec![0; 2]; n];
    arr[0][0] = 0;
    arr[0][1] = 1;

    for i in 1..n {
        arr[i][1] = arr[i - 1][0];
        arr[i][0] = arr[i - 1][0] + arr[i - 1][1];
    }

    println!("{}",arr[n - 1][0] + arr[n - 1][1]);
}
