pub mod metadata;
pub mod pipeline;
pub mod store;
pub mod tag_manager;
pub mod custom_field;
pub mod template;
pub mod auto_classify;

pub use auto_classify::AutoClassifier;
pub use custom_field::CustomFieldManager;
pub use metadata::AssetMetadata;
pub use pipeline::MetadataPipeline;
pub use store::MetadataStore;
pub use tag_manager::TagManager;
pub use template::TemplateManager;
