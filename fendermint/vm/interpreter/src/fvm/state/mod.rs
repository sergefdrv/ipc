// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

pub mod fevm;
pub mod ipc;
pub mod snapshot;

mod check;
mod exec;
mod genesis;
mod priority;
mod query;

use std::sync::Arc;

pub use check::FvmCheckState;
pub use exec::{BlockHash, FvmExecState, FvmStateParams, FvmUpdatableParams};
pub use genesis::{empty_state_tree, FvmGenesisState};
pub use query::FvmQueryState;

use super::{externs::FendermintExterns, store::ReadOnlyBlockstore};

pub use exec::FvmApplyRet;

/// We use full state even for checking, to support certain client scenarios.
// CheckStateRef is now generic over M to support different module types
pub type CheckStateRef<DB, M = fendermint_module::NoOpModuleBundle<DB, FendermintExterns<DB>>> =
    Arc<tokio::sync::Mutex<Option<FvmExecState<ReadOnlyBlockstore<DB>, M>>>>;
