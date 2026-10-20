use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        // 공백 문자로 구분
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // 메모리 기록
        if tokens[0] == "mem+" {
            memory += prev_result;
            print_output(memory);
            continue;
        } else if tokens[0] == "mem-" {
            memory -= prev_result;
            print_output(memory);
            continue;
        }

        // 수식 계산
        let left: f64 = if tokens[0] == "mem" {
            memory
        } else {
            tokens[0].parse().unwrap()
        };

        let right: f64 = if tokens[2] == "mem" {
            memory
        } else {
            tokens[2].parse().unwrap()
        };

        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                unreachable!()
            }
        };

        // 결과 표시
        print_output(result);
        prev_result = result;
    }
}

fn print_output(value: f64) {
    println!(" => {}", value);
}
