pub fn up_a_2_z() {
    ('A'..='z').for_each(|c| {
        if c.is_ascii_alphanumeric() {
            print!("{}", c)
        }
    });
}
