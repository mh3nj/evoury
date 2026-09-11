pub mod dock;
pub mod panel;
pub mod workspace;

pub use dock::{DockArea, DockConfig, DockNode, SplitDirection};
pub use panel::{PanelConfig, PanelId, PanelState};
pub use workspace::{LayoutProfile, LayoutRegistry};
