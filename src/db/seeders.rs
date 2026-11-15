use chrono::{Datelike, Days, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr};

use crate::entities::surveys::{self, Entity as Survey};

pub async fn seed_db(db: &DatabaseConnection) -> Result<(), DbErr> {
    let now = Utc::now();
    let year = now.year();

    let existing_survey = Survey::find_by_year(year).one(db).await?;
    if existing_survey.is_none() {
        let new_survey = surveys::ActiveModel {
            slug: Set(format!("nds-{year}")),
            year: Set(year),
            title: Set(format!("Nigeria Developer Survey {year}")),
            opens_at: Set(Some(
                now.checked_add_days(Days::new(30)).unwrap().fixed_offset(),
            )),
            closes_at: Set(Some(
                now.checked_add_days(Days::new(90)).unwrap().fixed_offset(),
            )),
            ..Default::default()
        };

        let survey = new_survey.insert(db).await?;

        println!("Insert survey {:?}", survey);
    }
    Ok(())
}
