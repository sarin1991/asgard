use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir("src/protobuf_messages");
    prost_build.compile_protos(&["protos/asgard_messages.proto"], &["protos/"])?;
    Ok(())
}
