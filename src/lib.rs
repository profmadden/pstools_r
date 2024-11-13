pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn psversion() -> String {
    "PSTools_R version 1.0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
