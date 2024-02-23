fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/tinykvpb.proto")?;
    // tonic_build::compile_protos("proto/schedulerpb.proto")?;
    // tonic_build::compile_protos("proto/raft_serverpb.proto")?;
    // tonic_build::compile_protos("proto/raft_cmdpb.proto")?;
    // tonic_build::compile_protos("proto/metapb.proto")?;
    // tonic_build::compile_protos("proto/kvrpcpb.proto")?;
    // tonic_build::compile_protos("proto/errorpb.proto")?;
    // tonic_build::compile_protos("proto/eraftpb.proto")?;
    // tonic_build::compile_protos("proto/coprocessor.proto")?;

    // tonic_build::configure()
    //     .build_server(true)
    //     .build_client(true)
    //     .out_dir("proto")
    //     .compile(&["proto/helloworld.proto"], &["proto"])?;
    //

    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}