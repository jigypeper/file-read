use std::fs;

fn main() {
    let contents = fs::read_to_string("./DOB.txt").expect("File path does not exist");
    let vec_contents: Vec<Vec<_>> = contents
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    println!("Name | DOB\n=========================");
    let mut data = String::from("Name | DOB\n=========================");
    vec_contents.iter().for_each(|p| {
        println!("{} | {}", p[0..2].join(" "), p[p.len() - 3..].join(" "));
        data.push_str(&format!(
            "\n{} | {}",
            p[0..2].join(" "),
            p[p.len() - 3..].join(" ")
        ))
    });
    let _ = fs::write("./output.txt", data);
}
