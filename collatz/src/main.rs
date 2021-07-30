use collatz::collatz;

fn main() {
    for i in 1..10 {
        println! {"New loop, start with {}",i};
        let steps = collatz(i, 1);
        println! {"Starting: {}, Steps to 1: {}",i,steps}
    }
}
