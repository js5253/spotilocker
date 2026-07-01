use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Default, PartialEq, Eq, FromRow)]
pub struct Model {
    pub id: u32,
    pub username: String,
    pub password: String
    // #[sea_orm(has_one)]
    // pub fruit: Option<super::fruit::Entity>,
    // #[sea_orm(has_many, via = "cake_filling")]
    // pub fillings: Vec<super::filling::Entity>,
}
