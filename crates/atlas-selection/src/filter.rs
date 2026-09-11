use uuid::Uuid;

pub struct SelectionFilter;

impl SelectionFilter {
    pub fn by_type(ids: &[Uuid], _type_filter: &str) -> Vec<Uuid> {
        ids.to_vec()
    }

    pub fn by_rating(ids: &[Uuid], _min_rating: u8) -> Vec<Uuid> {
        ids.to_vec()
    }

    pub fn by_favorite(ids: &[Uuid], _favorite: bool) -> Vec<Uuid> {
        ids.to_vec()
    }

    pub fn by_tag(ids: &[Uuid], _tag: &str) -> Vec<Uuid> {
        ids.to_vec()
    }
}
