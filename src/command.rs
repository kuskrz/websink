use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    /// port number (W_PORT)
    #[arg(short = 'p', long = "port", default_value_t = 2024)]
    pub port: u16,

    /// key file (W_KEY)
    #[arg(short = 'k', long = "key")]
    pub key: Option<String>,

    /// cert file (W_CERT)
    #[arg(short = 'c', long = "cert")]
    pub cert: Option<String>,

    /// do not produce stdout (W_NOOUT)
    #[arg(
        short = 'n',
        long = "noout",
        action(ArgAction::SetTrue),
        default_value_t = false
    )]
    pub noout: bool,

    /// body maximum size in bytes (W_BYTES)
    #[arg(short = 'b', long = "bytes", default_value_t = 10240)]
    pub bytes: usize,

    /// response file path (W_RESPONSE)
    #[arg(short, long)]
    pub response: Option<String>,

    /// additional response delay in ms, max 120000 (W_DELAY)
    #[arg(short, long, default_value_t = 0)]
    pub delay: u32,

    /// sink mode - do nothing, respond with 200, other options ignored (W_SINK)
    #[arg(
        short = 's',
        long = "sink",
        action(ArgAction::SetTrue),
        default_value_t = false
    )]
    pub sink: bool,
}
