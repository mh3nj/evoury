pub mod v001_initial;

pub const ALL_MIGRATIONS: &[Migration] = &[v001_initial::MIGRATION];

pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
    pub rollback: Option<&'static str>,
}
