use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

fn main() {
    write("Name", "123456")
}

fn write(key: &str, value: &str) {
    let path = Path::new("data/data.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    let file_size = fs::metadata(path).unwrap().len();
    let mut pair = format!("{key}:{value}");

    if file_size != 0 {
        pair = format!(",{pair}");
    }
    file.write_all(pair.as_bytes()).unwrap();
}
