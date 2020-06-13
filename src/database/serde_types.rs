use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[ derive (Debug, Deserialize, Serialize) ]
pub struct FileDataRecord {

	pub path: PathBuf,
    pub size: u64,

    #[ serde (skip_serializing_if = "Option::is_none") ]
    pub content_hash: Option <String>,

    #[ serde (skip_serializing_if = "Option::is_none") ]
	pub content_hash_time: Option <i64>,

    #[ serde (skip_serializing_if = "Option::is_none") ]
	pub extent_hash: Option <String>,

    #[ serde (skip_serializing_if = "Option::is_none") ]
	pub extent_hash_time: Option <i64>,

    #[ serde (skip_serializing_if = "Option::is_none") ]
	pub defragment_time: Option <i64>,

    #[ serde (skip_serializing_if = "Option::is_none") ]
	pub deduplicate_time: Option <i64>,

	pub mtime: i64,
	pub ctime: i64,

	pub mode: u32,
    pub uid: u32,
    pub gid: u32,

}

// ex: noet ts=4 filetype=rust
