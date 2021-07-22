use ed25519_dalek_blake3::PublicKey;
use smallvec::SmallVec;
use std::net::Ipv4Addr;

pub struct Node {
  addr: Ipv4Addr,
  key: PublicKey,
}

struct Kad {
  id: [u8; 32],
  bucket: SmallVec<[SmallVec<[Node; 16]>; 32]>,
}

impl Kad {
  pub fn add(node: Node) -> bool {
    let mut distance = 0;
    if distance > 64 {
      distance = 64;
    }
    false
  }
  pub fn neighbor(node: Node) -> bool {
    false
  }
}