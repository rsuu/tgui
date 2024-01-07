use thiserror::Error;

pub type Res<T> = Result<T, MyErr>;

#[derive(Debug, Error)]
pub enum MyErr {
    #[error("TODO")]
    Todo,

    #[error("nix")]
    Errno(#[from] nix::errno::Errno),

    #[error("prost")]
    Decode(#[from] prost::DecodeError),

    #[error("std::io")]
    StdIo(#[from] std::io::Error),

    #[error("ProtoZeroLen")]
    ProtoZeroLen,
}
