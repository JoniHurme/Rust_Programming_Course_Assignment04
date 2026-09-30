pub struct BookLoan {
    pub(crate) title: String,
    pub(crate) borrower: String,
    pub(crate) days_remaining: u32,
}

impl BookLoan {

    // An associated function named new that accepts a title and a borrower,
    // then creates a loan with 14 days remaining.
    pub fn new(title: String, borrower: String) -> BookLoan {
        BookLoan{
            title,
            borrower,
            days_remaining: 14,
        }
    }

    // A summary method that borrows the loan immutably (&self) and prints
    // its title, borrower, and remaining days.
    pub fn summary(&self) {
        println!("{} is borrowed by {} for {} more days.", self.title, self.borrower, self.days_remaining);
    }

    // A pass_day method that borrows the loan mutably (&mut self) and
    // reduces days_remaining by one.
    pub fn pass_day(&mut self) {

        // Check if there are still days remaining.
        if self.days_remaining > 0 {
            self.days_remaining -= 1;
        }
    }

}
