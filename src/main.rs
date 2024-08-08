use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    stdin().read_line(&mut input).unwrap();

    let arr: Vec<isize> = input.split_ascii_whitespace().flat_map(str::parse::<isize>).collect();
    let mut l_arr = vec![0; n];
    let mut r_arr = vec![0; n];
    l_arr[0] = arr[0];
    r_arr[n - 1] = arr[n - 1];

    let mut result: isize = l_arr[0];
    for i in 1..n {
        l_arr[i] = max(arr[i], l_arr[i - 1] + arr[i]);
        result = max(result, l_arr[i]);
    }

    if n > 1 {
        for i in (0..=n - 2).rev() {
            r_arr[i] = max(arr[i], r_arr[i + 1] + arr[i]);
        }
    }

    for i in 1..n - 1 {
        let temp = l_arr[i - 1] + r_arr[i + 1];
        result = max(temp, result);
    }
    println!("{result}\n");
}
