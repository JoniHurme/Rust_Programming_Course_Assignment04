mod book;

fn main() {

    let mut stalker = book::BookLoan {
        title: String::from("Stalker"),
        borrower: String::from("John Doe"),
        days_remaining: 14,
    };
    stalker.pass_day();
    stalker.summary();

    println!("\n");

    let mut loan = book::BookLoan::new(String::from("Metro 2033"), String::from("Jane Doe"));

    loan.summary();

    for _i in 0..150 {
        loan.pass_day();
    }

    println!();

    loan.summary();
}
