use supernovas_sys as sn;

fn main() {
    unsafe  {
        sn::novas_use_cspice();
        sn::cspice_add_kernel(std::env::var("EPH_DE405").unwrap().as_ptr() as *const i8);
    }
}