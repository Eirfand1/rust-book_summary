use std::collections::HashMap;

fn main() {
    let mut num: Vec<i32> = vec![10, 5, 5, 1, 2, 2, 6, 17, 2];
    num.sort();

    let nilai_tengah = median(&num);
    let modus = modus(&num);

    println!("{:#?}", &num);
    println!("Nilai tengahnya adalah : {}", nilai_tengah);
    println!("modusnya adalah : {}", modus);
}

fn median(n: &[i32]) -> i32 {
    let total: usize = n.len();

    match total % 2 {
        0 => (n[total / 2 - 1] + n[total / 2]) / 2,
        _ => n[total / 2],
    }
}

fn modus(n: &[i32]) -> String {
    let mut map = HashMap::new();

    for i in n {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = map.iter().max_by_key(|entry| entry.1).unwrap();
    format!("{}", max_value.0)
}
