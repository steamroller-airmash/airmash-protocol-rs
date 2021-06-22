/// Sent periodically by the client to indicate that it is still alive to the
/// server.
#[derive(Copy, Clone, Debug, Default)]
pub struct Ack;

/// Request a detailed score packet from the server.
#[derive(Copy, Clone, Debug, Default)]
pub struct ScoreDetailed;
