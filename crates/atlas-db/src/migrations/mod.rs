pub mod v001_initial;
pub mod v002_library_index;

pub const ALL_MIGRATIONS: &[Migration] = &[v001_initial::MIGRATION, v002_library_index::MIGRATION];

pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
    pub rollback: Option<&'static str>,
}
