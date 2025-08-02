use sea_orm::{ActiveValue, IntoActiveValue, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "camelCase")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(8))",
    rename_all = "snake_case"
)]
pub enum Gender {
    Male,
    Female,
}

impl IntoActiveValue<Gender> for Gender {
    fn into_active_value(self) -> ActiveValue<Gender> {
        ActiveValue::Set(self)
    }
}
