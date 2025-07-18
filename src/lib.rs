pub fn test(name: &str) -> String{
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test(){
        let result = test("test");
        assert_eq!(result, "Hello, test!");
    }
}