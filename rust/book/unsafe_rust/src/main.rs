mod raw_ptrs;
mod functions;
mod static_var;

fn main() {
    raw_ptrs::raw_pointer();
    functions::call_unsafe();
    static_var::change_static();
}
