fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let mut values = Vec::<Value>::new();

    loop {
        let next = lines.next();
        if next.is_none() {
            break;
        }

        values.push(parse_list(next.unwrap()));
        values.push(parse_list(lines.next().unwrap()));

        lines.next(); // blank line
    }

    values.push(Value::List(vec![Value::List(vec![Value::Integer(2)])]));
    values.push(Value::List(vec![Value::List(vec![Value::Integer(6)])]));

    values.sort_by(|l, r| to_ordering(compare(l, r)));

    let i = values.iter().position(|v| {
        compare(v, &Value::List(vec![Value::List(vec![Value::Integer(2)])])) == Status::Equal
    });
    let j = values.iter().position(|v| {
        compare(v, &Value::List(vec![Value::List(vec![Value::Integer(6)])])) == Status::Equal
    });

    println!("{}", (i.unwrap() + 1) * (j.unwrap() + 1));
}

#[derive(Debug)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

#[derive(Debug, PartialEq)]
enum Status {
    Equal,
    Valid,
    Invalid,
}

fn to_ordering(status: Status) -> std::cmp::Ordering {
    match status {
        Status::Equal => std::cmp::Ordering::Equal,
        Status::Valid => std::cmp::Ordering::Less,
        Status::Invalid => std::cmp::Ordering::Greater,
    }
}

fn compare(left: &Value, right: &Value) -> Status {
    match (left, right) {
        (Value::List(l), Value::List(r)) => {
            for i in 0..std::cmp::max(l.len(), r.len()) {
                if i >= l.len() {
                    return Status::Valid;
                }
                if i >= r.len() {
                    return Status::Invalid;
                }
                let result = compare(&l[i], &r[i]);
                if result != Status::Equal {
                    return result;
                }
            }
            Status::Equal
        }
        (Value::Integer(l), Value::Integer(r)) => {
            if l < r {
                Status::Valid
            } else if l > r {
                Status::Invalid
            } else {
                Status::Equal
            }
        }
        (Value::Integer(l), r) => compare(&Value::List(vec![Value::Integer(*l)]), r),
        (l, Value::Integer(r)) => compare(l, &Value::List(vec![Value::Integer(*r)])),
    }
}

fn find_list(input: &str) -> &str {
    let mut bracket_count = 1;
    let mut index = 1;
    while bracket_count > 0 {
        if input.chars().nth(index) == Some('[') {
            bracket_count += 1;
        }
        if input.chars().nth(index) == Some(']') {
            bracket_count -= 1;
        }
        index += 1;
    }
    &input[..index]
}

fn parse_list(input: &str) -> Value {
    let mut vec: Vec<Value> = Vec::new();

    let mut index = 1;
    loop {
        if input.chars().nth(index).unwrap() == ']' {
            break;
        } else if input.chars().nth(index).unwrap() == '[' {
            let sub_list = find_list(&input[index..]);
            vec.push(parse_list(sub_list));
            index += sub_list.len() - 1;
        } else if input.chars().nth(index).unwrap() == ',' {
            // do nothing
        } else {
            let mut end = index;
            while input.chars().nth(end).unwrap().is_digit(10) {
                end += 1;
            }
            vec.push(Value::Integer((&input[index..end]).parse::<i32>().unwrap()));
            index = end - 1;
            if input.chars().nth(index).unwrap() == ']' {
                index -= 1;
            }
        }

        index += 1
    }

    Value::List(vec)
}
