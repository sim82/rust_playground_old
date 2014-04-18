
use std::io::BufferedReader;
use std::io::File;
use std::path::Path;

fn hash( s : &str ) -> u64 {
  let mut hash : u64 = 5381;

  for c in s.chars() {
    hash = ((hash<<5) + hash) ^ c as u64;
  }
 // for( size_t i = 0; i < s.size(); ++i ) {
 //   hash = ((hash << 5) + hash) ^ s[i]; /* hash * 33 + c */
 // }
  hash
}

struct HashBuilder {
  table_size : u64,
  link_offsets : ~[u64],	
  ptrs : ~[(u64,u64)],
  file_dest : ~[(~str,u64)],
  append_pos : u64
}

impl HashBuilder {
  fn add( &mut self, filename : &str ) -> () {
    let path = Path::new(filename);
    let size = path.stat().unwrap().size; 
    let name_hash = hash(filename);
    
    //let mut file = File::open(filename);

    let bucket = (name_hash % self.table_size) as uint;
    
    self.ptrs.push(( self.link_offsets[bucket], self.append_pos ) );
    // update 'next pointer'
    self.link_offsets[bucket] = self.append_pos;
  
  
    let mut file_pos = self.append_pos + 8;
    
    print!("{} {} {} {}\n", filename, size, name_hash, file_pos );
    
    self.file_dest.push( (filename.into_owned(), file_pos));

    file_pos += filename.len() as u64 + 1 + 8;
    //file_pos = align( file_pos );
//         std::cout << "file pos: " << file_pos << "\n";
    self.append_pos = file_pos + size;
    
    
  }
  
  fn new( table_size : u64 ) -> HashBuilder {
    
    HashBuilder{ table_size : table_size, 
		  link_offsets : std::slice::from_elem(table_size as uint, 0u64), 
		  ptrs : ~[], 
		  file_dest : ~[], 
		  append_pos : table_size * 8 
    }
  }
}

fn main() {
  let path = Path::new("files.txt");
  let mut file = BufferedReader::new(File::open(&path));

  let table_size = 8 * 1024;
  let mut builder = HashBuilder::new(table_size);

  for line in file.lines() {
    let line = line.unwrap();
    let line = line.trim_right();
    //print!("{} {}\n", line, hash(line));
    builder.add(line);
  }
  
}