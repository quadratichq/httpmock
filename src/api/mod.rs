// TODO: Remove this at some point
#![allow(clippy::needless_lifetimes)]

pub use adapter::{local::LocalMockServerAdapter, MockServerAdapter};

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cfg(feature = "remote")]
pub use adapter::remote::RemoteMockServerAdapter;

use crate::common;
pub use mock::{Mock, MockExt};
pub use proxy::Recording;
pub use server::MockServer;
pub use spec::{Then, When};

mod adapter;
mod mock;
mod output;
mod proxy;
mod server;
pub mod spec;

/// Type alias for [regex::Regex](../regex/struct.Regex.html).
pub type Regex = common::data::HttpMockRegex;

pub use crate::common::data::Method;
