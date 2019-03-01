# svpng

Rust version of [miloyip/svpng](https://github.com/miloyip/svpng).

## Features

totally the same as miloyip's `svpng`

## Usage

Either using the `svpng` crate or just copy the `src/lib.rs` somewhere you want.

## Examples

```rust
use svpng::svpng;

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

```
---

RGB

![rgb](rgb.png)

---

RGBA

![rgba](rgba.png)