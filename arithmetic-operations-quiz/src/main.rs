use rand::RngExt;

fn main() {
    let mut num_of_correct = 0;

    while num_of_correct < 3 {
        let quiz_mode = rand::rng().random_range(1..=2);

        match quiz_mode {
            1 => {
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);

                println!("{} + {} = ??", op1, op2);
                println!("??의 값을 입력하세요: ");
                let mut ans_input = String::new();

                // 표준 입력으로 한 줄 받아서 ans_input에 대입
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_input을 trim()으로 줄바꿈을 제거하고 parse()로 정수(u32) 타입으로 변환
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                dbg!(ans_input);
                if dbg!(ans_input == dbg!(op1) + dbg!(op2)) {
                    println!("정답");
                    num_of_correct += 1;
                } else {
                    println!("오답");
                }
            }
            2 => {
                let op1 = rand::rng().random_range(0..100);
                let op2 = rand::rng().random_range(0..100);

                println!("{} - {} = ??", op1, op2);
                println!("??의 값을 입력하세요: ");
                let mut ans_input = String::new();

                // 표준 입력으로 한 줄 받아서 ans_input에 대입
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_input을 trim()으로 줄바꿈을 제거하고 parse()로 정수(u32) 타입으로 변환
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                dbg!(ans_input);
                if dbg!(ans_input == dbg!(op1) - dbg!(op2)) {
                    println!("정답");
                    num_of_correct += 1;
                } else {
                    println!("오답");
                }
            }
            _ => unreachable!(),
        }
    }

    println!("클리어!");
}
