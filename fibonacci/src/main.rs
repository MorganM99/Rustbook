use std::env;

fn fibonacci(n:u32) -> u32{
    let mut res:u32=0;
    if n==0{
        return res;
    }
    else if n==1
    {
        res=1;
        return res;
    }
    else{
        res=fibonacci(n-1)+fibonacci(n-2);
        return res;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n=&args[1];
    let n:u32=n.trim().parse().expect("Please enter a number!");
    let res=fibonacci(n);
    println!("The {}th fibonacci number is: {}",n,res);
}
