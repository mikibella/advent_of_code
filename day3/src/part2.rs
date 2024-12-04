use regex::Regex;

pub fn process(input: &str) -> i32 {
    let mut result = 0;
    let mut skip_next_multiplications = false;
    // Check for mul(*,*) in string
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    for cap in re.captures_iter(input) {
        //check if needle is do() or don't() and set flag accordingly
        if cap[0].starts_with("do()") {
            skip_next_multiplications = false;
            continue;
        }
        if cap[0].starts_with("don't()") {
            skip_next_multiplications = true;
            continue;
        }

        //skip if don't() is set
        if skip_next_multiplications {
            continue;
        }
        // check if number has more than 3 digits
        if cap[2].len() > 3 || cap[3].len() > 3 {
            continue;
        }

        //compute value
        let num1 = match cap[2].parse::<i32>() {
            Ok(num1) => num1,
            Err(_) => continue,
        };
        let num2 = match cap[3].parse::<i32>() {
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(process(input), 48);
    }
}
