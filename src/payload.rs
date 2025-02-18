#[derive(Debug)]
pub struct Payload;

impl Payload {
    pub fn apply_spacing(value: &str) -> String {
        let mut with_spacing: String = String::new();
        let mut counter: u32 = 0;
        for c in value.chars() {
            with_spacing.push(c);
            counter += 1;
            if counter == 2 {
                with_spacing.push_str(&" ");
                counter = 0;
            }
        }
        with_spacing.trim().to_uppercase().to_string()
    }
}
