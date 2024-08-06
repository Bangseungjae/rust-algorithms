use std::io::stdin;

fn main() {
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();
    let n: usize = str.trim().parse().unwrap();
    let mut arr: Vec<u32> = vec![0; n + 1];
    arr[0] = 1;
    arr[1] = 1;


    for i in 2..=n {
        arr[i] = (arr[i - 1] + arr[i - 2]) % 10_007;
    }

    println!("{}", arr[n]);
}
