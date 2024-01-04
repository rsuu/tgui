fn main() {
    prost_build::compile_protos(&["src/tgui.proto0"], &["src"]).unwrap();
}
