pub mod protocol;
pub mod server;
pub mod fitness_tools;
pub mod nutrition_tools;
pub mod types;
pub mod transport;
pub mod auth;

pub use server::MCPServer;
pub use protocol::{MCPProtocol, MCPMessage, MCPRequest, MCPResponse, MCPError};
pub use fitness_tools::FitnessTools;
pub use nutrition_tools::NutritionTools;
pub use types::*;
pub use transport::Transport;
pub use auth::AuthManager;