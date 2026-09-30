mod book;

fn main() {

    // Created new bookloan directly using the struct.
    // let mut stalker = book::BookLoan {
    //     title: String::from("Stalker"),
    //     borrower: String::from("John Doe"),
    //     days_remaining: 14,
    // };
    // stalker.pass_day();
    // stalker.summary();
    //
    // println!("\n");

    // Created a new bookloan using the implemented new method.
    let mut loan = book::BookLoan::new(String::from("Metro 2033"), String::from("Jane Doe"));

    loan.summary();

    // Pass 150 days to demonstrate that the loan can't be a negative number.
    // for _i in 0..150 {
    //     loan.pass_day();
    // }
    for _i in 0..3{
        loan.pass_day();
    }

    println!();

    loan.summary();
}
