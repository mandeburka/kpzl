extern crate kpzl;

#[cfg(not(test))]
fn main() {
    kpzl::ui::play::<kpzl::super2048::Super2048>();
}
