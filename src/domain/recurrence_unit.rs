use chrono::{Local, Months, NaiveDate, TimeDelta};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DeriveActiveEnum, EnumIter};
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "recurrence_unit")]
pub enum RecurrenceUnit {
    #[sea_orm(string_value = "Days")]
    Days,
    #[sea_orm(string_value = "Weeks")]
    Weeks,
    #[sea_orm(string_value = "Month")]
    Months,
}

impl RecurrenceUnit {
    pub fn next(&self, naive_date: NaiveDate, interval: u32) -> NaiveDate {
        match self {
            RecurrenceUnit::Days => naive_date.add(TimeDelta::days(interval as i64)),
            RecurrenceUnit::Weeks => naive_date.add(TimeDelta::weeks(interval as i64)),
            RecurrenceUnit::Months => naive_date.add(Months::new(interval)),
        }
    }

    pub fn next_now(&self, interval: u32) -> NaiveDate {
        self.next(Local::now().date_naive(), interval)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_days() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert_eq!(
            RecurrenceUnit::Days.next(date, 10),
            NaiveDate::from_ymd_opt(2025, 1, 11).unwrap()
        );
    }

    #[test]
    fn test_next_weeks() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert_eq!(
            RecurrenceUnit::Weeks.next(date, 2),
            NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()
        );
    }
    #[test]
    fn test_next_months() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert_eq!(
            RecurrenceUnit::Months.next(date, 2),
            NaiveDate::from_ymd_opt(2025, 3, 1).unwrap()
        );
    }
}
