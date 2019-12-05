fn main() {
    part1();
    part2();
}

fn part1() {
    let lower = 246515;
    let upper = 739105;

    let mut num_passwords = 0;

    'main: for mut candidate in lower..upper + 1 {
        let mut prev = std::i32::MAX;
        let mut has_double = false;

        while candidate != 0 {
            let curr = candidate % 10;
            if curr == prev {
                has_double = true;
            }
            if curr > prev {
                continue 'main;
            }

            prev = curr;
            candidate /= 10;
        }

        if has_double {
            num_passwords += 1;
        }
    }

    println!("part1: {}", num_passwords);
}

fn part2() {
    let lower = 246515;
    let upper = 739105;

    let mut num_passwords = 0;

    'main: for mut candidate in lower..upper + 1 {
        let mut prev = std::i32::MAX;
        let mut has_double = false;
        let mut sequentially_matching = 1;

        while candidate != 0 {
            let curr = candidate % 10;
            if curr > prev {
                continue 'main;
            }

            if curr == prev {
                sequentially_matching += 1;
            } else {
                if sequentially_matching == 2 {
                    has_double = true;
                }
                sequentially_matching = 1;
            }

            prev = curr;
            candidate /= 10;
        }

        if sequentially_matching == 2 {
            has_double = true;
        }

        if has_double {
            num_passwords += 1;
        }
    }

    println!("part2: {}", num_passwords);

}
