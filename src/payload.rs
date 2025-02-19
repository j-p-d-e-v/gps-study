#[derive(Debug)]
pub struct Payload;

impl Payload {
    pub fn to_binary(value: &str) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = Vec::new();
        for v in value.split_whitespace() {
            match u8::from_str_radix(v, 16) {
                Ok(b) => {
                    bytes.push(b);
                }
                Err(error) => return Err(format!("{:?}", error)),
            }
        }
        Ok(bytes)
    }

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
