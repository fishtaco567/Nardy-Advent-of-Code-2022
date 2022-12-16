pub fn run() {
    let input = include_str!("../input/day_6.txt");

    let mut i = 0;
    let mut v = ['\0'; 14];

    let mut co = 0;
    for c in input.chars() {
        v[i] = c;

        let mut s = true;
        for c in v {
            let n = v.iter().filter(|f| **f == c).count();

            if n != 1 || c == '\0' {
                s = false;
            }
        }

        if s {
            println!("{}", co + 1);
            break;
        } else {
            i = (i + 1) % 14;
            co += 1;
        }
    }
}
