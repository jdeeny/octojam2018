use png;

use std::collections::HashMap;

use std::fs::File;

use std::fs::read_dir;
use std::path::Path;
use std::io::Write;


fn closest_color(colors: &HashMap<usize, (u8, u8, u8)>, color: (u8, u8, u8)) -> usize {
    let mut result = 0;
    let mut error = std::f64::MAX;
    for (n, c) in colors.iter() {
        let e = (color.0 as f64 - c.0 as f64) * (color.0 as f64 - c.0 as f64) +
                (color.1 as f64 - c.1 as f64) * (color.1 as f64 - c.1 as f64) +
                (color.2 as f64 - c.2 as f64) * (color.2 as f64 - c.2 as f64);
        if e < error {
            result = *n;
            error = e;
        }
    }
    return result;
}


fn process_sprite(header_file: &mut Write, data_file: &mut Write, colors: &HashMap<usize, (u8, u8, u8)>, name: &str, path: &Path) {

    println!("{:?} {:?}", name, path);


    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();


    for y in 0..info.height {
        write!(data_file, "# ");
        for x in 0..info.width {
            let idx: usize = ((y*info.width + x) * 4) as usize;
            let px_color = (buf[idx], buf[idx+1], buf[idx+2]);
            let px_alpha = buf[idx+3];
            if px_alpha < 100 {
                write!(data_file, "_");
            } else
            {
                let color = closest_color(colors, px_color);
                write!(data_file, "{}", color);
            }
        }
        writeln!(data_file, "");
    }



    writeln!(data_file, ": SPR_{}", name);
    for half in 0..2 {
        writeln!(data_file, "# Half {}", half);
        for segment in 0..3 {
            writeln!(data_file, "# Seg {}", segment);
            for plane in 0..2 {
                writeln!(data_file, "# Plane {}", plane);
                for y in 0..6 {
                    let px_y = y + segment * 6;
                    let px_y = px_y as i64 - (18-info.height as i64);
                    write!(data_file, "0b");
                    if px_y < 0 || px_y > info.height as i64 {
                        writeln!(data_file, "00000000");
                    } else {
                        println!("y: {}", px_y);
                        for x in 0..7 {
                            let px_x = x + half * 7;
                            if px_x > info.width {
                                write!(data_file, "0");
                            } else {
                                let idx: usize = ((px_y as usize * info.width as usize + px_x as usize) * 4) as usize;
                                //println!("{} {} {}", px_x, px_y, idx);
                                let c = closest_color(colors, (buf[idx], buf[idx+1], buf[idx+2]));
                                //println!("{} {} {} {} = {}", buf[idx], buf[idx+1], buf[idx+2], buf[idx+3], c);
                                if (c >> plane) != 0 && buf[idx+3] > 128 {
                                    write!(data_file, "1");
                                } else {
                                    write!(data_file, "0");
                                }
                            }
                        }

                        writeln!(data_file, "0");
                    }
                }
            }


            writeln!(data_file, "# Mask");
            for y in 0..6 {
                let px_y = y + segment * 6;
                let px_y = px_y as i64 - (18-info.height as i64);
                write!(data_file, "0b");
                if px_y < 0 || px_y > info.height as i64 {
                    writeln!(data_file, "00000000");
                } else {
                    println!("y: {}", px_y);
                    for x in 0..7 {
                        let px_x = x + half * 7;
                        if px_x > info.width {
                            write!(data_file, "0");
                        } else {
                            let idx: usize = ((px_y as usize * info.width as usize + px_x as usize) * 4) as usize;
                            let m = buf[idx+3];
                            if m > 128 {
                                write!(data_file, "1");
                            } else {
                                write!(data_file, "0");
                            }
                        }
                    }

                    writeln!(data_file, "0");
                }
            }

        }
    }



}


fn main() {
    let mut header_dest = File::create("build/sprite_header.o8").unwrap();
    let mut data_dest = File::create("build/sprite_data.o8").unwrap();

    let mut colors : HashMap<usize, (u8, u8, u8)> = HashMap::new();
    colors.insert(0, (22, 58, 92));
    colors.insert(1, (33, 41, 70));
    colors.insert(2, (97, 134, 169));
    colors.insert(3, (146, 27, 48));

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.

    let mut sprite_count = 0;


    if let Ok(entries) = read_dir("../../assets/enemies") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let sprite_name = entry.file_name();

                        let mut sprite_path = entry.path();
                        sprite_path.push(Path::new("tile.png"));
                        println!("{:?}", sprite_path);
                        writeln!(header_dest, ":const SPR_{} {}", sprite_name.to_string_lossy(), sprite_count);
                        sprite_count += 1;
                        process_sprite(&mut header_dest, &mut data_dest, &colors, &sprite_name.to_string_lossy(), &sprite_path);
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
                        writeln!(header_dest, ":const SPR_{} {}", sprite_name.to_string_lossy(), sprite_count);
                        sprite_count += 1;
                        process_sprite(&mut header_dest, &mut data_dest, &colors, &sprite_name.to_string_lossy(), &sprite_path);
                    }
                }
            }
        }
    }



}
