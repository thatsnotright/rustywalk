use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Chunk {
  timestamp: [u8; 4], // 4 bytes, big endian
  length: [u8; 3],
  type_id: u8,
  message_stream_id: [u8; 4], // 4 bytes in little endian
  payload: Vec<u8>,
}

fn to_chunk(timestamp: u32, type_id: u8, message_stream_id: u32, payload: Vec<u8>) -> Chunk {
  let len = payload.len().to_be_bytes();
  Chunk {
    timestamp: timestamp.to_be_bytes(),
    length: len[0..2]
      .try_into()
      .expect("length has an incorrect number of bytes"),
    type_id,
    message_stream_id: message_stream_id.to_le_bytes(),
    payload,
  }
}

struct ChunkStream {
  timestamp: u32,
  type_id: u8,
  message_stream_id: [u8; 4],
}

struct ServerHandshake {
  version: u8,
  time: u32,
}

struct Context {
  sender: tokio::sync::broadcast::Sender<Chunk>,
  receiver: tokio::sync::broadcast::Receiver<Chunk>,
  server: ServerHandshake,
}

struct StateMachine<S> {
  state: S,
  context: Context,
  chunk_streams: Vec<ChunkStream>,
}

struct Error;
struct Start;
struct C0 {
  version: u8,
}
struct C1 {
  time: u32,
  zero: u32,
  random: [u8; 1528],
}
struct S0 {
  version: u8,
}
struct S1 {
  time: u32,
  zero: u32,
  random: [u8; 1528],
}
struct C2;
struct S2;

trait Next<T, U> {
  fn next(&self, val: StateMachine<T>) -> Result<StateMachine<U>, Error>;
}
impl Next<Start, C0> for StateMachine<Start> {
  fn next(&self, val: StateMachine<Start>) -> Result<StateMachine<C0>, Error> {
    let next_state = C0 { version: 3 };

    Ok(StateMachine {
      state: next_state,
      context: val.context,
      chunk_streams: val.chunk_streams,
    })
  }
}
impl Next<C0, C1> for StateMachine<C0> {
  fn next(&self, val: StateMachine<C0>) -> Result<StateMachine<C1>, Error> {
    let mut random = [0u8; 1528];
    // This is not strictly necessary per the spec, the data must only appear random
    rand::thread_rng().fill_bytes(&mut random);
    let next_state = C1 {
      time: 0,
      zero: 0,
      random,
    };
    Ok(StateMachine {
      chunk_streams: val.chunk_streams,
      state: next_state,
      context: val.context,
    })
  }
}
// impl Next<C1, S1> for StateMachine<C1> {
//   fn next(&self, val: StateMachine<C1>) -> Result<StateMachine<S1>, Error> {}
// }
