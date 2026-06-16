const FOLDERS: &[(&'static str, &'static str, bool); 1] = &[
    ("src/resources/asm/", "eldenring", false),
];

fn main() {
    utils::object::build(FOLDERS)
}