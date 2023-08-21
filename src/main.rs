use clap::Parser;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type ReplicaId = i128;
type ClientId = i128;
type OperationNumber = i128;
type CommitNumber = i128;
type RequestNumber = i128;

struct Replica {
    id: ReplicaId,
    addr: &'static str,
}

struct Config {
    replicas: Vec<Replica>,
}

enum ReplicaStatus {
    Normal,
    ViewChange,
    Recovering,
}

enum RequestStatus {
    Success,
    Failure,
}

struct RequestResult {}

struct RequestState {
    number: RequestNumber,
    status: RequestStatus,
    result: RequestResult,
}

struct State {
    config: Config,
    status: ReplicaStatus,
    op_number: OperationNumber,
    commit_number: CommitNumber,
    requests_log: HashMap<OperationNumber, Payload>,
    client_state: HashMap<ClientId, RequestState>,
}

impl State {
    pub fn default(config: Config) -> Self {
        Self {
            config,
            status: ReplicaStatus::Normal,
            op_number: OperationNumber::default(),
            commit_number: CommitNumber::default(),
            client_state: HashMap::default(),
            requests_log: HashMap::default(),
        }
    }
}

/// TODO: The operation the client wants to run
struct Operation {}

/// The data sent from any client to the primary replica.
struct Payload {
    /// The operation the client wants to run
    operation: Operation,
    /// The client sending the operation
    client_id: ClientId,
    /// The number assigned to the request
    request_number: RequestNumber,
}

fn prepare() {
    todo!()
}

/// Handles the request received by the primary replica.
async fn handle_client_request(state: Arc<Mutex<State>>, payload: Payload) -> Result<(), String> {
    // If the received request number is smaller than the one on the `client_state`, the request
    // is ignored.
    let mut state_guard = state.lock().await;
    let client_request_state = state_guard.client_state.get(&payload.client_id).unwrap();

    // TODO: fix this with anyhow
    // The request number must be larger than the last one received by the replica.
    let _ = match payload.request_number < client_request_state.number {
        true => Ok(()),
        false => Err(String::from("request number is outdated")),
    }?;

    let current_op_number = state_guard.op_number.clone() + 1;

    state_guard.op_number = current_op_number;

    state_guard.requests_log.insert(current_op_number, payload);

    Ok(())
}

fn main() {
    let replicas = vec![
        Replica {
            id: 0,
            addr: "0.0.0.0:3000",
        },
        Replica {
            id: 1,
            addr: "0.0.0.0:3001",
        },
        Replica {
            id: 2,
            addr: "0.0.0.0:3002",
        },
        Replica {
            id: 3,
            addr: "0.0.0.0:3003",
        },
    ];

    let config = Config { replicas };

    let state = State::default(config);

    let config = println!("Hello, world!");
}
