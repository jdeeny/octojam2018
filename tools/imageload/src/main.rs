use png;

use std::collections::HashMap;


fn main() {

    use std::fs::File;

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open("../../assets/enemies/hero1/tile.png").unwrap());
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
