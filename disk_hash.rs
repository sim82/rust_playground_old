
use std::io::BufferedReader;
use std::io::File;
use std::path::Path;
use std::libc;

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

fn align( v : u64 ) -> u64 {
    let a = 32; // FIXME: magic number
    
    v + (a - (v % a)) % a
}

struct NameOffsetPair {
  name : ~str,
  offset : u64
}

struct HashBuilder {
  table_size : u64,
  next_pointers : ~[u64],	
  chain_links : ~[(u64,u64)],
  file_dest : ~[NameOffsetPair],
  //file_dest : ~[(~str,u64)],
  append_pos : u64
}

impl HashBuilder {
  fn write( &self, filename : &str ) -> () {
    // print!( "create\n");
    let mut file = File::create(&Path::new(filename));

    // write table size to the end of the file
    file.seek(self.append_pos as i64, std::io::SeekSet).unwrap();
    file.write_le_u64(self.table_size).unwrap();

    // copy the files to the output file
    for p in self.file_dest.iter() {

      let name : (&str) = p.name;
      let offset = p.offset; 
      // print!( "write: {}\n", name );

      file.seek(offset as i64, std::io::SeekSet).unwrap();

      // write name (0 terminated)
      file.write_str(name).unwrap();
      file.write_u8(0).unwrap();

      let path = Path::new(name);
      let size = path.stat().unwrap().size as u64; 
      let mut in_file = File::open(&path);
      let data = in_file.read_to_end().unwrap();

      // write file size
      file.write_le_u64(size).unwrap();
      let out_pos = align(file.tell().unwrap()) as i64;

      // write file content
      file.seek(out_pos, std::io::SeekSet).unwrap();
      file.write(data).unwrap();
    } 

    // write hash chains (i.e., the 'next pointers')
    for p in self.chain_links.iter() {
      let (offs, v) = *p;
      
      file.seek(offs as i64, std::io::SeekSet).unwrap();
      file.write_le_u64(v).unwrap();
    }
  }


  fn add( &mut self, filename : &str ) -> () {
    // print!( "add file {}\n", filename );
    let path = Path::new(filename);
    if !(path.is_file() && path.exists()) {
      print!( "ignore: {}\n", filename);
      return
    }

    // calculate hash-value / bucket
    let size = path.stat().unwrap().size; 
    let name_hash = hash(filename);
    
    let bucket = (name_hash % self.table_size) as uint;
    
    // add link to the bucket-chain
    self.chain_links.push(( self.next_pointers[bucket], self.append_pos ) );

    // update 'next pointer' for the bucket (i.e., the tail of the bucket-chain)
    self.next_pointers[bucket] = self.append_pos;
  
    let mut file_pos = self.append_pos + 8;
    
    // store filename/offset pair for writing later
    //self.file_dest.push( (filename.into_owned(), file_pos ));
    self.file_dest.push( NameOffsetPair{ name : filename.into_owned(), offset : file_pos });

    // calculate total space occupied by file / metadata and update append_pos
    file_pos += filename.len() as u64 + 1 + 8;
    file_pos = align( file_pos );
    self.append_pos = file_pos + size;
    
    
  }
  
  fn new( table_size : u64 ) -> HashBuilder {
    let mut next_pointers : ~[u64] = ~[];

    for i in range(0, table_size) {
      next_pointers.push( i * 8 );
    }

    HashBuilder{ table_size : table_size, 
		  next_pointers : next_pointers, //std::slice::from_elem(table_size as uint, 0u64), 
		  chain_links : ~[], 
		  file_dest : ~[], 
		  append_pos : table_size * 8 
    }
  }
}


// struct DiskHash {

// }

// struct FileDescriptor(libc::c_int);

// impl Drop for FileDescriptor {
//     fn finalize(&self) { unsafe { libc::close(**self); } }
// }

// unsafe fn open(filename : &str) -> FileDescriptor {
//     let fd = libc::open(filename.as_ptr(), libc::O_RDONLY as libc::c_int, 0);
    
//     if fd < 0 {
//         fail!(format!("failure in open({}): {}", filename, std::os::last_os_error()));
//     }
//     return FileDescriptor(fd);
// }

// fn print_chain( map : &std::os::MemoryMap, offs : u64 ) {
//   let mut next_offs = 0;
//   let mut name : (~str);
//   unsafe {
//     std::slice::raw::buf_as_slice(map.data.offset(offs as int) as *u64, 1, |sl| {
//       next_offs = sl[0];
//     });
//     name = std::str::raw::from_c_str( map.data.offset((offs + 8) as int) as *std::libc::c_char );


//   }
//   print!( " -> {} ", name );
//   if next_offs != 0 {
//     print_chain( map, next_offs);
//   } else {
//     println!("");
//   }

// }

fn test_mmap() {
  let filename = "hash.bin";
  let path = Path::new(filename);
  let size = path.stat().unwrap().size;

  let fd = unsafe {libc::open(filename.as_ptr() as *i8, libc::O_RDONLY as libc::c_int, 0)};

  let map = std::os::MemoryMap::new(size as uint, [std::os::MapReadable, std::os::MapFd(fd)]).unwrap();

  let table_size = 8 * 1024;

  unsafe {
    std::slice::raw::buf_as_slice(map.data as *u64, table_size, |buckets| {
      for i in range(0, table_size) {
        if buckets[i] != 0 {

          let mut offs = buckets[i];

          print!( "{}", offs );

          while offs != 0 {
            let mut name : (~str);
            name = std::str::raw::from_c_str( map.data.offset((offs + 8) as int) as *std::libc::c_char );

            // is there a better way to load a single u64 from a raw pointer? I guess this will crash on cpus with strict alignment requirements?
            std::slice::raw::buf_as_slice(map.data.offset(offs as int) as *u64, 1, |sl| {
              offs = sl[0];
            });
            print!( " -> {} ", name );
          }
          println!("");
        }
      }
    });
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
  builder.write("hash.bin");
  
  test_mmap();
}