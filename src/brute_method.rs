const MAX_LIMIT: u32 = 100;

fn find_if_working(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    vec: &mut Vec<Vec<u32>>,
) -> (bool, &mut Vec<Vec<u32>>) {
    let mut algo_works: bool = false;
    if a.pow(3) + b.pow(3) == c.pow(3) + d.pow(3) {
        vec.push(vec![a, b, c, d]);
        algo_works = true;
    }
    (algo_works, vec)
}

fn loop_through() -> Vec<Vec<u32>> {
    let mut possible_combinations: Vec<Vec<u32>> = vec![];
    let mut a;
    let mut b;
    let mut c;
    let mut d;
    for a_int in 1..MAX_LIMIT {
        a = a_int as u32;
        for b_int in 1..MAX_LIMIT {
            b = b_int as u32;
            for c_int in 1..MAX_LIMIT {
                c = c_int as u32;
                for d_int in 1..MAX_LIMIT {
                    d = d_int as u32;
                    let (algo_works, _) = find_if_working(a, b, c, d, &mut possible_combinations);
                    // there is only one possible d for any combination of a, b and c
                    if algo_works {
                        break;
                    }
                }
            }
        }
    }

    possible_combinations
}

pub fn run_brute() {
    let result = loop_through();
    // println!("{:?}", result.len());
}
