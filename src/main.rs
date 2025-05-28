use std::fs::File;
use std::io::{BufWriter, Write};

const N: u128 = 10_000_000_000;

fn main() -> std::io::Result<()> {
    let file = File::create("output.bin")?;
    let mut writer = BufWriter::new(file);
    let mut x: u64 = 10442988001049997600;
    for _ in 1..N {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        x = x.wrapping_mul(0x2545F4914F6CDD1Du64);
        writer.write_all(&x.to_ne_bytes())?;
    }
    Ok(())
}
