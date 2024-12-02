pub fn process() -> Result<i32, anyhow::Error> {
    Ok(0)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process();
        assert_eq!(result.unwrap(), 0);
    }
}
