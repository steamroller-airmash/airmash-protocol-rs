use crate::types::Player;

#[derive(Clone, Debug)]
pub struct ChatWhisper {
  pub from: Player,
  pub to: Player,
  pub text: String,
}
