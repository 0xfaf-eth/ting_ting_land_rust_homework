pub fn a_2_up_z() {
    ('a'..='z').for_each(|c| {
        if c.is_ascii_alphanumeric() {
            print!("{}", c)
        }
    });
    ('A'..='Z').for_each(|c| {
        if c.is_ascii_alphanumeric() {
            print!("{}", c)
        }
    });
}
