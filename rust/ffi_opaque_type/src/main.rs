#[repr(C)]
pub struct Opaque {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub struct OpaqueContainer {
    o: *mut Opaque,
}

impl OpaqueContainer {
    pub fn new() -> Self {
        Self {
            o: unsafe { make_new_opaque() },
        }
    }
}

impl Drop for OpaqueContainer {
    fn drop(&mut self) {
        println!("drop me");
        unsafe { delete_opaque(self.o) }
    }
}

extern "C" {
    pub fn make_new_opaque() -> *mut Opaque;
    pub fn print_name(o: *mut Opaque);
    pub fn delete_opaque(o: *mut Opaque);
}

fn main() {
    {
        let c = OpaqueContainer::new();
        unsafe {
            print_name(c.o);
        }
    }
    unsafe {
        let o = make_new_opaque();
        print_name(o);
        delete_opaque(o);
    }
    println!("Hello, world!");
}
