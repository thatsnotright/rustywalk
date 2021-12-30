pub trait Actor {
  fn new(is_frozen: bool, w: usize, h: usize) -> Self;
}
