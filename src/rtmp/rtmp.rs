use byteorder::BigEndian;
use rand::prelude::*;
use serde::{
  ser,
  ser::{SerializeStruct, Serializer},
  Deserialize, Serialize,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context as TaskContext, Poll};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Chunk {
  pub payload_only: bool,
  timestamp: Option<[u8; 4]>, // 4 bytes, big endian
  length: Option<[u8; 3]>,
  type_id: Option<u8>,
  message_stream_id: Option<[u8; 4]>, // 4 bytes in little endian
  pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InitializationChunk {
  payload: Vec<u8>,
}

fn to_chunk(timestamp: u32, type_id: u8, message_stream_id: u32, payload: Vec<u8>) -> Chunk {
  let len = payload.len().to_be_bytes();
  Chunk {
    payload_only: false,
    timestamp: Some(timestamp.to_be_bytes()),
    length: Some(
      len[0..2]
        .try_into()
        .expect("length has an incorrect number of bytes"),
    ),
    type_id: Some(type_id),
    message_stream_id: Some(message_stream_id.to_le_bytes()),
    payload,
  }
}
struct ChunkStream {
  timestamp: u32,
  type_id: u8,
  message_stream_id: [u8; 4],
}

pub struct ServerHandshake {
  pub version: u8,
  pub time: u32,
}

pub struct Context {
  pub sender: tokio::sync::broadcast::Sender<Chunk>,
  pub server: ServerHandshake,
  pub rand: [u8; 1528],
}

pub struct RTMPClient {
  pub state: States,
  pub context: Box<Context>,
  chunk_streams: Box<Vec<ChunkStream>>,
}

impl RTMPClient {
  pub fn new(context: Context) -> Self {
    RTMPClient {
      state: States::Start,
      context: Box::new(context),
      chunk_streams: Box::new(vec![]),
    }
  }
}

enum States {
  Start,
  C0 {
    version: u8,
  },
  C1 {
    time: u32,
    zero: u32,
    random: [u8; 1528],
  },
  S0 {
    version: u8,
  },
  S1 {
    time: u32,
    zero: u32,
    random: [u8; 1528],
  },
  C2,
  S2,
}
pub struct Error;

impl ser::Serialize for States {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    match self {
      States::Start => {}
      States::C0 { version } => serializer.serialize_u8(*version),
      States::C1 { time, zero, random } => {
        let mut state = serializer.serialize_struct("C1", 1528 + 4 + 4)?;
        serializer.serialize_bytes(&u32::to_be_bytes(*time));
        serializer.serialize_bytes(&[0; 4]);
        serializer.serialize_bytes(random);
        state.end()
      }
      _ => {}
    }
  }
}

pub struct RTMPSerializer {
  output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Error>
where
  T: Serialize,
{
  let mut serializer = RTMPSerializer { output: Vec::new() };
  value.serialize(serializer)?;
  Ok(serializer.output)
}
impl<'a> ser::Serializer for &'a mut RTMPSerializer {
  type Ok = ();
  type Error = Error;
  
   `SerializeSeq`, `SerializeTuple`, `SerializeTupleStruct`, `SerializeTupleVariant`, `SerializeMap`, `SerializeStruct`, `SerializeStructVariant`, `serialize_bool`, `serialize_i8`, `serialize_i16`, `serialize_i32`, `serialize_i64`, `serialize_u8`, `serialize_u16`, `serialize_u32`, `serialize_u64`, `serialize_f32`, `serialize_f64`, `serialize_char`, `serialize_str`, `serialize_bytes`, `serialize_none`, `serialize_some`, `serialize_unit`, `serialize_unit_struct`, `serialize_unit_variant`, `serialize_newtype_struct`, `serialize_newtype_variant`, `serialize_seq`, `serialize_tuple`, `serialize_tuple_struct`, `serialize_tuple_variant`, `serialize_map`, `serialize_struct`, `serialize_struct_variant`

}

pub trait Next<RTMPClient> {
  fn next(self, payload: Vec<u8>) -> Result<RTMPClient, Error>;
}
impl Next<RTMPClient> for RTMPClient {
  fn next(self, payload: Vec<u8>) -> Result<RTMPClient, Error> {
    let next_state = self.state;

    match self.state {
      Start => {
        next_state = States::C0 { version: 3 };
        self.context.sender.send(Chunk {
          payload_only: true,
          timestamp: None,
          length: None,
          message_stream_id: None,
          type_id: None,
          payload: to_bytes(&next_state)?,
        });
      }
      C0 => {
        let mut random = [0u8; 1528];
        // This is not strictly necessary per the spec, the data must only appear random
        rand::thread_rng().fill_bytes(&mut random);
        next_state = States::C1 {
          time: 0,
          zero: 0,
          random,
        };
      }
      _ => {}
    }

    Ok(RTMPClient {
      state: next_state,
      context: self.context,
      chunk_streams: self.chunk_streams,
    })
  }
}

impl Future for States {
  type Output = ();
  fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext) -> Poll<()> {
    use States::*;
    loop {
      match *self {
        Start => *self = C0 { version: 3 },
      }
    }
  }
}
