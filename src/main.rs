mod minecraft;
pub mod common;

fn main() {
    let _ = minecraft::forge::versions::get_versions();
}