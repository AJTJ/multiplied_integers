use std::collections::HashMap;

// find all solutions of a^3 + b^3 = c^3 + d^3
// where 0 < a,b,c,d < 1000
// ex: all same numbers is correct
// ex: they mirror each other

// brute force:
// where we loop through every possible integer. O(n^4)

// optimization
// a + b = c + d

// simple algo to find working equations
// avoiding integer overflow here
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

fn bap() -> () {}

// a fn that loops over all possible integers and checks for successful equations
// this is looping through the range of integers
// with 3 internal loops
// VERY brute force
fn loop_through() -> Vec<Vec<u32>> {
    let mut shared: Vec<Vec<u32>> = vec![];
    let mut a;
    let mut b;
    let mut c;
    let mut d;
    for a_int in 1..999 {
        a = a_int as u32;
        for b_int in 1..999 {
            b = b_int as u32;
            for c_int in 1..999 {
                c = c_int as u32;
                for d_int in 1..999 {
                    d = d_int as u32;
                    let (algo_works, _) = find_if_working(a, b, c, d, &mut shared);
                    // there is only one possible d for any combination of a, b and c
                    if algo_works {
                        break;
                    }
                }
            }
        }
    }

    shared
}

// creating a hash of c^3 + d^3 = (all possible iterations of c and d)
// use a HashMap
// for this we would loop through c and d once
fn create_hash_table() -> HashMap<u32, Vec<(u32, u32)>> {
    let mut c_d_hash: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for x in 1..999 {
        for y in 1..999 {
            let sum: u32 = (x as u32).pow(3) + (y as u32).pow(3);
            match c_d_hash.get_mut(&sum) {
                Some(val) => {
                    // add to hash val
                    val.push((x, y));
                }
                None => {
                    // insert new hash
                    c_d_hash.insert(sum, vec![(x, y)]);
                }
            }
        }
    }
    c_d_hash
}

fn main() {
    let c_d_hash = create_hash_table();
    println!("{:?}", c_d_hash);
    // println!("{:?}", loop_through());
}
