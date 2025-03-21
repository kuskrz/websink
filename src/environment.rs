use crate::command::CmdArgs;
use std::env;

pub fn set_from_env(cmdargs: &mut CmdArgs) {
    if let Ok(e) = env::var("W_PORT") {
        if cmdargs.port == 2024 {
            if let Ok(v) = e.parse::<u16>() {
                cmdargs.port = v;
            }
        }
    }
    if let Ok(e) = env::var("W_KEY") {
        if cmdargs.key.is_none() {
            cmdargs.key = Some(e);
        }
    }
    if let Ok(e) = env::var("W_CERT") {
        if cmdargs.cert.is_none() {
            cmdargs.cert = Some(e);
        }
    }
    if env::var("W_NOOUT").is_ok() {
        cmdargs.noout = true;
    }
    if let Ok(e) = env::var("W_BYTES") {
        if cmdargs.bytes == 10240 {
            if let Ok(v) = e.parse::<usize>() {
                cmdargs.bytes = v;
            }
        }
    }
    if let Ok(e) = env::var("W_RESPONSE") {
        if cmdargs.response.is_none() {
            cmdargs.response = Some(e);
        }
    }
    if let Ok(e) = env::var("W_DELAY") {
        if cmdargs.delay == 0 {
            if let Ok(v) = e.parse::<u32>() {
                cmdargs.delay = v;
            }
        }
    }
    if env::var("W_SINK").is_ok() {
        cmdargs.sink = true;
    }
}
