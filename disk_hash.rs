
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
  link_offsets : ~[u64],	
  ptrs : ~[(u64,u64)],
  file_dest : ~[NameOffsetPair],
  append_pos : u64
}

impl HashBuilder {
  fn write( &self, filename : &str ) -> () {
    //for( std::vector< std::pair< std::string, size_t > >::iterator it = file_dest_.begin(); it != file_dest_.end(); ++it ) {
    for p in self.file_dest.iter() {
      //let (name, append_pos) : (~str, u64) = *p; 
      let name = p.name.clone();
      let offset = p.offset; 
    }      
        //     // write name (0 terminated)
        //     const std::string &name = it->first;
        //     char *append_pos = base + it->second;
        //     std::copy( name.begin(), name.end(), append_pos );
           
        //     append_pos += name.size();
        //     *append_pos = 0; ++append_pos;
            
        //     // write file size
        //     std::ifstream is( it->first.c_str(), std::ios::binary );
        //     assert( is.good() );

        //     is.seekg( 0, std::ios::end );
        //     size_t size = is.tellg();
        //     is.seekg( 0, std::ios::beg );
        //     {
        //         int_type x = size;
        //         char *xcp = (char*)&x;
        //         std::copy( xcp, xcp + int_size, append_pos );
        //     }
        //     append_pos += int_size;
            
        //     // write file content
        //     append_pos = align( append_pos );
        //     is.read( append_pos, size );
        // }
        
        // // write hash chains (i.e., the 'next pointers')
        // for( std::vector< std::pair< size_t, size_t > >::iterator it = chain_links_.begin(); it != chain_links_.end(); ++it ) {
        //     int_type x = it->second;
        //     char *xcp = (char*)&x;
            
        //     std::copy( xcp, xcp + int_size, base + it->first );
        // }

  }


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
    
    self.file_dest.push( NameOffsetPair{ name : filename.into_owned(), offset : file_pos });

    file_pos += filename.len() as u64 + 1 + 8;
    file_pos = align( file_pos );
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
  builder.write("disk.hash");
  
}