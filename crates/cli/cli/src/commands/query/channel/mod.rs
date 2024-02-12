// mod end;
// mod ends;
// mod client;

// pub use end::QueryChannelEnd;
// pub use ends::QueryChannelEnds;
// pub use client::QueryClientEnds;

use crate::Result;

#[derive(Debug, clap::Subcommand)]
pub enum QueryChannel {
    End(QueryChannelEnd),
    Ends(QueryChannelEnds),
    Client(QueryClientEnd),
}

impl QueryChannel {
    pub async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        match self {
            Self::End(cmd) => cmd.run(builder).await,
            Self::Ends(cmd) => cmd.run(builder).await,
            Self::Client(cmd) => cmd.run(builder).await,
        }
    }
}
