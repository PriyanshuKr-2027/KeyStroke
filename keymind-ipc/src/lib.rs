pub mod protocol;
pub mod server;

pub use protocol::{
    DailyStatsDto, IpcRequest, IpcResponse, LearnedPhraseDto, VariableDto,
};
pub use server::IpcServer;
