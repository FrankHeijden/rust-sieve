use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let upper = args.get(1).expect("Please enter an upper bound!");
    let upper: usize = upper.trim().parse().expect("Please enter a valid number!");

    sieve_of_eratosthenes(upper);
}

fn sieve_of_eratosthenes(n: usize) {
    let mut prime = vec![true; n + 1];

    let upper: usize = (n as f64).sqrt() as usize + 1;
    for p in 2..upper  {
        if prime[p] {
            println!("{}", p);
            for i in (p * 2..n).step_by(p) {
                prime[i] = false;
            }
        }
    }

    for p in upper..n {
        if prime[p] {
            println!("{}", p);
        }
    }
}
