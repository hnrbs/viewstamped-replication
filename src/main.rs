use std::collections::HashMap;

type ReplicaId = i128;
type ClientId = i128;
type OperationNumber = i128;
type CommitNumber = i128;
type RequestNumber = i128;

struct Replica {
    id: ReplicaId,
    addr: String,
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
    client_state: HashMap<ClientId, RequestState>,
}

fn main() {
    println!("Hello, world!");
}
