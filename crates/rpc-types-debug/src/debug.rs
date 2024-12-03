//! Types for the `debug` API.

use alloy_primitives::Bytes;
use serde::{Deserialize, Serialize};

/// Represents the execution witness of a block. Contains an optional map of state preimages.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionWitness {
    /// Trie nodes that were required during the execution of
    /// the block, including during state root recomputation.
    pub state: Vec<Bytes>,
    /// Contract bytecode preimages that were required during
    /// the execution of the block, including during state root recomputation.
    pub codes: Vec<Bytes>,
    /// Account and storage trie node preimages that were required during
    /// the execution of the block.
    pub keys: Vec<Bytes>,
}
