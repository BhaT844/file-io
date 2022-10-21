use std::io;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

use rand::Rng;

/*
하루는 24시간이다. 24시간제 시계에서는 하루가 자정에서 시작해서 다음 날 자정에서 끝나며,
자정에서부터 지금까지 흐른 시간을 기준으로 시각을 표기한다. 예를 들어, 지금이 오후 8시라면 “20시”와 같은 꼴로 표현할 수 있다.
자정을 표기하는 유일한 방법은 “0시”임에 유의하라.
지금은 자정에서부터 정확히 A시간이 지났다. 앞으로 정확히 B시간이 더 지난다면, 24시간제 시계에서 그 때는 몇 시일까?
*/

fn main() {
    const INPUT_DATA_AMOUNT: usize = 100_000;

    println!("(1: input모드 2: output모드)");
    let select_mod = select();

    let now = Instant::now();

    if select_mod == "1" {
        insert_input_data(INPUT_DATA_AMOUNT);
    } else if select_mod == "2" {
        generate_output();
    }

    let elapsed = now.elapsed();
    println!("경과시간: {:.3?}", elapsed);
}

fn select() -> String {
    let mut k = String::new();

    io::stdin().read_line(&mut k)
        .expect("입력중 오류");

    k.trim().to_string()
}

fn insert_input_data(line: usize) {
    let mut input_file = File::create("input.txt")
        .expect("파일 생성 실패");

    for _ in 0..line {
        let a = rand::thread_rng().gen_range(1, 24);
        let b = rand::thread_rng().gen_range(1, 101);
        let data = format!("{} {}\n", a, b);

        input_file.write_all(data.as_bytes())
            .expect("파일 작성중 오류");
    }

    println!("input파일 작성 완료!");
}

fn generate_output() {
    let input_file = File::open("input.txt").expect("input파일 불러오기 실패");
    let mut output_file = File::create("output.txt").expect("파일 생성 실패");
    let buffered = BufReader::new(input_file);
    let mut line_num = 1_usize;

    for line in buffered.lines() {
        match line {
            Ok(data) => {
                let vec = data.split_whitespace()
                    .map(|s| s.trim().parse().expect("정수타입으로 변환하지 못했습니다."))
                    .collect::<Vec<usize>>();

                let a = vec[0];
                let b = vec[1];

                let result = format!("#{} {} {} -> {}\n", line_num, a, b, (a + b) % 24);
                output_file.write_all(result.as_bytes()).expect("파일 작성중 오류");
            },
            Err(_) => panic!("파일 작성중 오류"),
        }
        line_num += 1;
    }

    println!("output파일 작성 완료!");
}