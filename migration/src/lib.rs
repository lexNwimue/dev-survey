pub use sea_orm_migration::prelude::*;

mod helpers;
mod m20261024_234908_create_survey_table;
mod m20261025_004250_add_updated_at_trigger;
mod m20261025_010000_create_users_table;
mod m20261025_010100_create_sections_table;
mod m20261025_010200_create_questions_table;
mod m20261025_010300_create_user_surveys_table;
mod m20261025_010400_create_responses_table;
mod m20261115_035124_survey_year;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261024_234908_create_survey_table::Migration),
            Box::new(m20261025_004250_add_updated_at_trigger::Migration),
            Box::new(m20261025_010000_create_users_table::Migration),
            Box::new(m20261025_010100_create_sections_table::Migration),
            Box::new(m20261025_010200_create_questions_table::Migration),
            Box::new(m20261025_010300_create_user_surveys_table::Migration),
            Box::new(m20261025_010400_create_responses_table::Migration),
            Box::new(m20261115_035124_survey_year::Migration),
        ]
    }
}
