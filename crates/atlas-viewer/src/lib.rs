pub mod descriptor;
pub mod registry;
pub mod viewer;

pub use descriptor::ViewerDescriptor;
pub use registry::ViewerRegistry;
pub use viewer::{ViewerProvider, ViewerCapability, ViewerContext};
