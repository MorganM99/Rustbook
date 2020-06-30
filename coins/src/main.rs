#[derive(Debug)]
enum PoundStyle {
    Round,
    TwelveSided,
}

#[derive(Debug)]
enum Coin {
    OnePence,
    TwoPence,
    FivePence,
    TenPence,
    TwentyPence,
    FiftyPence,
    OnePound(PoundStyle),
    TwoPounds,
}

impl Coin {
    fn value_in_pence(&self) -> u32 {
        match *self {
            Coin::OnePence => 1,
            Coin::TwoPence => 2,
            Coin::FivePence => 5,
            Coin::TenPence => 10,
            Coin::TwentyPence => 20,
            Coin::FiftyPence => {
                println!("Right, who put 50p in the dickhead?");
                50
            }
            Coin::OnePound(ref pound_style) => {
                match pound_style{
                    PoundStyle::Round=>println!("Round pound!"),
                    PoundStyle::TwelveSided=>println!("Twelve sided pound!"),
                }
                100
            }
            Coin::TwoPounds => 200,
        }
    }
}

fn main() {
    let twenty = Coin::TwentyPence;
    let fifty = Coin::FiftyPence;
    let old_pound = Coin::OnePound(PoundStyle::Round);
    let new_pound = Coin::OnePound(PoundStyle::TwelveSided);
    println!("{}", twenty.value_in_pence());
    println!("{}", fifty.value_in_pence());
    println!("{}", old_pound.value_in_pence());
    println!("{}", new_pound.value_in_pence());
}
