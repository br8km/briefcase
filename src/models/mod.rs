// Models module for Briefcase
// Contains all data structures and entities

pub mod config;
pub mod backup;
pub mod sync;
pub mod crypto;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

