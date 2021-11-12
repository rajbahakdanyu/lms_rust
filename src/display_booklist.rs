use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_booklist() {
    let path = Path::new("booklist.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let temp: Vec<&str> = s.split('\n').collect();
            println!("Book Id\tBook Name\tAuthor\t\tQuantity\tPrice");
            for elem in temp {
                let temp2: Vec<&str> = elem.split(',').collect();
                println!(
                    "{}\t{}\t{}\t{}\t\t{}",
                    temp2[0], temp2[1], temp2[2], temp2[3], temp2[4]
                );
            }
            println!("");
        }
    }
}
