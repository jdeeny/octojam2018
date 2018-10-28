use png;

use std::collections::HashMap;

use std::fs::File;

use std::fs::read_dir;
use std::path::Path;
use std::io::Write;

fn process_sprite(header_file: &mut Write, data_file: &mut Write, name: &str, path: &Path) {

    println!("{:?} {:?}", name, path);

    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();

    let mut colors = HashMap::<(u8, u8, u8, u8), usize>::new();
    let mut count = 0;
    for y in 0..info.height {
        for x in 0..info.width {
            let idx: usize = ((y*info.width + x) * 4) as usize;
            let color = (buf[idx], buf[idx+1], buf[idx+2], buf[idx+3]);
            if colors.get(&color) == None {
                colors.insert(color, count);
                count += 1;
            }
        }
    }

    for c in colors {
        println!("{:?}", c);

    }

}


fn main() {
    let mut header_dest = File::create("build/sprite_header.o8").unwrap();
    let mut data_dest = File::create("build/sprite_data.o8").unwrap();


    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.



    if let Ok(entries) = read_dir("../../assets/enemies") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let sprite_name = entry.file_name();

                        let mut sprite_path = entry.path();
                        sprite_path.push(Path::new("tile.png"));
                        println!("{:?}", sprite_path);
                        process_sprite(&mut header_dest, &mut data_dest, &sprite_name.to_string_lossy(), &sprite_path);
                    }
                }
            }
        }
    }


    if let Ok(entries) = read_dir("../../assets/terrain") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let sprite_name = entry.file_name();

                        let mut sprite_path = entry.path();
                        sprite_path.push(Path::new("tile.png"));
                        println!("{:?}", sprite_path);
                        process_sprite(&mut header_dest, &mut data_dest, &sprite_name.to_string_lossy(), &sprite_path);
                    }
                }
            }
        }
    }



}
