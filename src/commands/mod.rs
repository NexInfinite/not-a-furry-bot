pub mod misc;
pub mod admin;
pub mod wahs;

pub use crate::commands::misc::echo::echo_mod::ECHO_COMMAND;
pub use crate::commands::misc::embed::embed_mod::EMBED_COMMAND;

pub use crate::commands::admin::role::role_mod::ROLE_COMMAND;
pub use crate::commands::admin::rrole::rrole_mod::RROLE_COMMAND;

pub use crate::commands::wahs::boop::boop_mod::BOOP_COMMAND;
pub use crate::commands::wahs::redpanda::redpanda_mod::REDPANDA_COMMAND;