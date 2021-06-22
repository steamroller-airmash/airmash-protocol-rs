/// Response packet to server
/// [`Ping`](../server/struct.ping.html)s.
#[derive(Copy, Clone, Debug)]
pub struct Pong {
  /// The ping number, should correspond
  /// to the `num` field within in the
  /// [`Ping`](../server/ping.html) packet
  /// sent by the server.
  pub num: u32,
}
