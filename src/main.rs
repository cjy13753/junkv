use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

fn main() {
    let path = Path::new("data/data.txt");
    write(path, "Name", "123456");
    read(path);
}

fn read(path: &Path) {
    let mut content = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    for kv in content.split(',') {
        println!("{}", kv);
    }
}

fn write(path: &Path, key: &str, value: &str) {
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
