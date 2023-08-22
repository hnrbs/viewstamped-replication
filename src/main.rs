use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type ReplicaId = i128;
type ClientId = i128;
type OperationNumber = i128;
type CommitNumber = i128;
type RequestNumber = i128;

#[derive(Clone, Debug)]
struct Replica {
    id: ReplicaId,
    addr: &'static str,
}

#[derive(Clone, Debug)]
struct Config {
    replicas: Vec<Replica>,
}

#[derive(Clone, Debug)]
enum ReplicaStatus {
    Normal,
    ViewChange,
    Recovering,
}

#[derive(Clone, Debug)]
enum RequestStatus {
    Success,
    Failure,
}

#[derive(Clone, Debug)]
struct RequestResult {}

#[derive(Clone, Debug)]
struct ClientState {
    request_number: RequestNumber,
    request_status: RequestStatus,
    request_result: RequestResult,
}

type ViewNumber = i128;

#[derive(Clone, Debug)]
struct State {
    config: Config,
    status: ReplicaStatus,
    operation_number: OperationNumber,
    commit_number: CommitNumber,
    requests_log: HashMap<OperationNumber, Payload>,
    client_state: HashMap<ClientId, ClientState>,
    view_number: ViewNumber,
}

impl State {
    pub fn default(config: Config) -> Self {
        Self {
            config,
            status: ReplicaStatus::Normal,
            operation_number: OperationNumber::default(),
            commit_number: CommitNumber::default(),
            client_state: HashMap::default(),
            requests_log: HashMap::default(),
            view_number: ViewNumber::default(),
        }
    }
}

/// The operation the client wants to run
// TODO: implement it
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Operation {}

/// The data sent from any client to the primary replica.
#[derive(Clone, Debug)]
struct Payload {
    /// The operation the client wants to run
    operation: Operation,
    /// The client sending the operation
    client_id: ClientId,
    /// The number assigned to the request
    request_number: RequestNumber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PreparePayload {
    view_number: ViewNumber,
    operation: Operation,
    operation_number: OperationNumber,
    commit_number: CommitNumber,
}

fn prepare(payload: PreparePayload) -> Result<(), String> {
    todo!();
}

/// Handles the request received by the primary replica.
async fn handle_client_request(state: Arc<Mutex<State>>, payload: Payload) -> Result<(), String> {
    // If the received request number is smaller than the one on the `client_state`, the request
    // is ignored.
    let mut state_guard = state.lock().await;
    let client_state = (*state_guard)
        .client_state
        .get(&payload.client_id)
        .unwrap()
        .clone();

    // TODO: fix this with anyhow
    // The request number must be larger than the last one received by the replica.
    let _ = match payload.request_number < client_state.request_number {
        true => Ok(()),
        false => Err(String::from("request number is outdated")),
    }?;

    // Update the primary operation number
    let current_operation_number = state_guard.operation_number.clone() + 1;
    state_guard.operation_number = current_operation_number;

    // Insert the request into the log
    state_guard
        .requests_log
        .insert(current_operation_number, payload.clone());

    // Update the request number
    let new_client_state = ClientState {
        request_number: payload.request_number,
        ..client_state
    };
    state_guard
        .client_state
        .insert(payload.client_id, new_client_state);

    // Send a prepare request to all the other replicas
    let replicas = (*state_guard).config.replicas.clone();

    let client = reqwest::Client::new();

    let prepare_payload = PreparePayload {
        view_number: state_guard.view_number,
        operation: payload.operation,
        operation_number: state_guard.operation_number,
        commit_number: state_guard.commit_number,
    };

    // TODO: don't send to itself
    for replica in replicas.iter() {
        let prepare_url = format!("{}/prepare", replica.addr);

        let _ = client
            .post(prepare_url)
            .body(serde_json::to_string(&prepare_payload).unwrap())
            .send()
            .await;
    }

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
