//! SigmaOS Desktop Environment
//! Main desktop environment with window management, panels, docks

pub mod desktop;
pub mod compositor;
pub mod shell;
pub mod cinnamon;

pub use desktop::DesktopEnvironment;
