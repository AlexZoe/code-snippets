use std::io::prelude::*;
use std::fs::File;

struct Coord2D {
    x: i32,
    y: i32,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

fn main() -> std::io::Result<()> {
    let coord = Coord2D { x: 13, y: -2};

    let mut fd = File::create("test.bin")?;

    fd.write_all( unsafe {any_as_u8_slice(&coord)} )?;

    Ok(())
}
