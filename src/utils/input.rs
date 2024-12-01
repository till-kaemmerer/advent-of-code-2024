fn get_input_string(path: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn get_input_lines(path: &str) -> Vec<String> {
    get_input_string(path)
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect()
}

pub fn get_input_numbers(path: &str) -> (Vec<i32>, Vec<i32>) {
    get_input_lines(path)
        .iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}
