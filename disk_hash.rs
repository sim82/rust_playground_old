extern crate meh;

use std::io::BufferedReader;
use std::io::File;
use std::path::Path;

use meh::HashBuilder;
use meh::DiskHash;

fn main() {

  let table_size = 8 * 1024;
  let mut builder = HashBuilder::new(table_size);

  let path = Path::new("files.txt");
  let mut file = BufferedReader::new(File::open(&path));

  for line in file.lines() {
    let line = line.unwrap();
    let line = line.trim_right();
    //print!("{} {}\n", line, hash(line));
    builder.add(line);
  }
  builder.write("hash.bin");
  
  // test_mmap();
  let dh_path = Path::new("hash.bin");
  let dh = DiskHash::new(&dh_path);
  let mut file = BufferedReader::new(File::open(&path));

  let mut not_found = 0i;

  for line in file.lines() {
    let line = line.unwrap();
    let line = line.trim_right();
    // let offs = dh.lookup(line);

    // println!("{} {}", line, offs );
    dh.lookup(line).as_slice(|x| {
      if x.len() == 0 {
        not_found += 1;
        return;
      }
      // print!("{}: ", line);
      // for i in range(0,10) {
      //   print!("{} ", x[i] as char);
      // }
    });
    // println!("");
  }
  println!("not found: {}", not_found);

}