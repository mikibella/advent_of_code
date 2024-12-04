use regex::Regex;

pub fn process(input: &str) -> i32 {
    let mut result = 0;
    // Check for mul(*,*) in string
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(input) {
        if cap[1].len() > 3 || cap[2].len() > 3 {
            continue;
        }
        let num1 = match cap[1].parse::<i32>() {
            Ok(num1) => num1,
            Err(_) => continue,
        };
        if cap[2].len() > 3 || cap[2].len() > 3 {
            continue;
        }
        let num2 = match cap[2].parse::<i32>() {
            Ok(num2) => num2,
            Err(_) => continue,
        };
        result += num1 * num2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process(input), 161);
    }
}
