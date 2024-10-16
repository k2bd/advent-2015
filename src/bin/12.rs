use serde_json::Value;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .split(['[', ']', '{', '}', ':', ','])
            .filter_map(|val| match val.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None,
            })
            .sum(),
    )
}

fn score_content(content: Value) -> i32 {
    match content.as_object() {
        Some(obj) => {
            if obj.values().filter_map(|v| v.as_str()).any(|v| v == "red") {
                0
            } else {
                obj.values().map(|c| score_content(c.clone())).sum()
            }
        }
        None => match content.as_array() {
            Some(arr) => arr.iter().map(|v| score_content(v.clone())).sum(),
            None => match content.as_i64() {
                Some(n) => n as i32,
                None => 0,
            },
        },
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(score_content(serde_json::from_str(input).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn test_part_one(#[case] json: &str, #[case] expected: i32) {
        let result = part_one(json);
        assert_eq!(result, Some(expected));
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_part_two(#[case] json: &str, #[case] expected: i32) {
        let result = part_two(json);
        assert_eq!(result, Some(expected));
    }
}
