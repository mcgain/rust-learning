use std::collections::HashMap;

fn main() {
    println!("You're Mean");
    let mut vector = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9];
    let mut mode_hash: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;
    for num in vector.iter() {
        total += *num;
        mode_hash.entry(*num).and_modify(|e| *e += 1).or_insert(0);
    }
    let mean = total / vector.len() as i32;
    vector.sort();
    let maybe_median = vector.get(vector.len() / 2);
    match maybe_median {
        None => panic!(),
        Some(median) => println!("median is {}", median),
    }

    println!("total is {}", total);
    println!("mean is {}", mean);

    let mut mode_score = -1;
    let mut mode = -1;
    for (key, value) in mode_hash.iter() {
        if *value > mode_score {
            mode_score = *value;
            mode = *key;
        }
    }
    println!("mode is {}", mode);
}
