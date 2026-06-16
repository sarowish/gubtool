const FOLDERS: &[(&'static str, &'static str, bool); 2] = &[
    ("src/sys/asm32/", "sys32", true),
    ("src/sys/asm64/", "sys64", false),
];

fn main() {
    utils::object::build(FOLDERS)
}