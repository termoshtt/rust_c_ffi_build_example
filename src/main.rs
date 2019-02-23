#[repr(C)]
pub struct DATA {
    count: u32,
}

#[link(name = "test", kind = "static")]
extern "C" {
    fn new_data() -> *mut DATA;
    fn del_data(data: *mut DATA);
}

fn main() {
    unsafe {
        let raw = new_data();
        del_data(raw);
    }
}
