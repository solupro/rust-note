fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOld)]");
    config.out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}