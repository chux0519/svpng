use ::svpng::svpng;

use std::io;

fn main() -> io::Result<()> {
    {
        // RGB
        let mut pix = Vec::new();
        for y in 0..=255 {
            for x in 0..=255 {
                pix.push(x);
                pix.push(y);
                pix.push(128);
            }
        }
        svpng("rgb.png", 256, 256, &pix, false)?;
    }

    {
        // RGBA
        let mut pix = Vec::new();
        for y in 0..=255 {
            for x in 0..=255 {
                pix.push(x);
                pix.push(y);
                pix.push(128);
                pix.push(x / 2 + y / 2);
            }
        }
        svpng("rgba.png", 256, 256, &pix, true)?;
    }

    Ok(())
}
