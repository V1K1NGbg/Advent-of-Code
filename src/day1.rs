use std::{io::{self, BufRead, BufReader}, fs::File};

fn read_lines(path: &str) -> io::Result<BufReader<File>> {
    let file =  File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    Ok(reader)
}
fn main() {
    let lines = read_lines("src/food.txt").expect("You need a str");
    let mut cal = 0;
    let mut max = 61971;
    let mut max_three = 0;

    for line in lines.lines(){
        let line = line.expect("IDK");
        if line != "" {
            cal = cal + line.parse::<i32>().unwrap();
        } else {
            if max <= cal {
                max = cal;
                max_three = max_three + cal;
                println!("{}",max);
            }

            cal = 0;
        }
        
    }
    println!("\n{}", max_three);
}
