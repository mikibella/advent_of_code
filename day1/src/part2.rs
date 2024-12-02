use std::collections::HashMap;

pub fn process(input: &str) -> Result<usize, anyhow::Error> {
    let mut left: Vec<usize> = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse()?);
        // if entry is present insert 1 of not modify value by adding 1
        right
            .entry(line.next().unwrap().parse()?)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let result: usize = left.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum();
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = process(input);
        assert_eq!(result.unwrap(), 31);
    }
}
