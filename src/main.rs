#! [allow (dead_code, unused_imports)]

mod receiver;

use vfs_demo::{VirtualFile, VFS};
use std::process::Command;
pub use camino::{Utf8Component, Utf8Components, Utf8Path, Utf8PathBuf, Utf8Prefix};
use core::error;
use std::{error::Error, env, fs::{self, File}, path::{Path, PathBuf}, io::{self, Write}};
use indexmap::{IndexMap, IndexSet};
use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher; 
use lsp_types::OneOf;
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types;

use receiver::main_loop;

// fn main() {

//     let file_name: Vec<String> = env::args().collect();
//     //let current_file = VirtualFile::from(file_name[1].clone());
//     let vfs = VFS::new();
//     let vfs = vfs.append(file_name[1].clone());
//     vfs.files_copy().expect("err");

//     VFS::compile("/Users/zhen/Rust/vfs-demo", "helloworld.rs");
//     VFS::run("/Users/zhen/Rust/vfs-demo", "helloworld");
// }

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };


    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}


