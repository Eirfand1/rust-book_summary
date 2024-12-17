use std::{collections::HashMap, io};

fn main() {
    let mut map = HashMap::new();
    loop {
        println!("Please enter a command:");
        println!("1. Add [Name] to [Department]");
        println!("2. List [Department]");
        println!("3. List All");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input invalid");
        let input = input.trim();

        match input {
            "1" => {
                let mut departement = String::new();
                let mut user = String::new();

                println!("Masukan Department : ");
                io::stdin()
                    .read_line(&mut departement)
                    .expect("Gagal membca input");
                println!("Masukan orang : ");

                io::stdin()
                    .read_line(&mut user)
                    .expect("Gagal membaca input");

                let user = user.trim().to_string();
                let departement = departement.trim().to_string();

                map.entry(departement.clone())
                    .or_insert_with(Vec::new)
                    .push(user.clone());
                println!("Data yang dimasukan = {} ; {}", departement, user);
            }
            "2" => {
                for (index, (key, _)) in map.iter().enumerate() {
                    println!("{}. {}", index + 1, key);
                }

                loop {
                    println!("Masukan Department : ");
                    let mut chose = String::new();
                    io::stdin().read_line(&mut chose).expect("gagal");
                    let chose = chose.trim();

                    if map.contains_key(chose) {
                        println!("{:#?}", map.get(chose));
                    } else {
                        println!("Department gk ada");
                    }
                    break;
                }
            }
            "3" => {
                for (index, (key, value)) in map.iter().enumerate() {
                    println!("{}. {}{:#?}", index + 1, key, value);
                }
            }
            "4" => break,
            _ => println!("gagl"),
        }
    }
}
