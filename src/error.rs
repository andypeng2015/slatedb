use std::{path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SlateDBError {
    #[error("IO error: {0}")]
    IoError(#[from] Arc<std::io::Error>),

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Empty SSTable")]
    EmptySSTable,

    #[error("Empty block metadata")]
    EmptyBlockMeta,

    #[error("Empty block")]
    EmptyBlock,

    #[error("Object store error: {0}")]
    ObjectStoreError(#[from] Arc<object_store::Error>),

    #[error("Manifest file already exists")]
    ManifestVersionExists,

    #[error("Failed to find manifest with id {0}")]
    ManifestMissing(u64),

    #[error("Failed to find latest manifest")]
    LatestManifestMissing,

    #[error("Invalid deletion")]
    InvalidDeletion,

    #[error("Invalid sst error: {0}")]
    InvalidFlatbuffer(#[from] flatbuffers::InvalidFlatbuffer),

    #[error("Invalid DB state error")]
    InvalidDBState,

    #[error("Invalid Compaction")]
    InvalidCompaction,

    #[error(
        "Invalid clock tick, most be monotonic. Last tick: {}, Next tick: {}",
        last_tick,
        next_tick
    )]
    InvalidClockTick { last_tick: i64, next_tick: i64 },

    #[error("Detected newer DB client")]
    Fenced,

    #[error("Invalid cache part size bytes, it must be multiple of 1024 and greater than 0")]
    InvalidCachePartSize,

    #[error("Invalid Compression Codec")]
    InvalidCompressionCodec,

    #[error("Error Decompressing Block")]
    BlockDecompressionError,

    #[error("Error Compressing Block")]
    BlockCompressionError,

    #[error("Unknown RowFlags -- this may be caused by reading data encoded with a newer codec")]
    InvalidRowFlags,

    #[error("Error flushing immutable wals: channel closed")]
    WalFlushChannelError,

    #[error("Error flushing memtables: channel closed")]
    MemtableFlushChannelError,

    #[error("Read channel error: {0}")]
    ReadChannelError(#[from] tokio::sync::oneshot::error::RecvError),
}

impl From<std::io::Error> for SlateDBError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(Arc::new(value))
    }
}

impl From<object_store::Error> for SlateDBError {
    fn from(value: object_store::Error) -> Self {
        Self::ObjectStoreError(Arc::new(value))
    }
}

/// Represents errors that can occur during the database configuration.
///
/// This enum encapsulates various error conditions that may arise
/// when parsing or processing database configuration options.
#[derive(Error, Debug)]
pub enum DbOptionsError {
    #[error("Unknown configuration file format: {0}")]
    UnknownFormat(PathBuf),

    #[error("Invalid configuration format: {0}")]
    InvalidFormat(#[from] figment::Error),
}
