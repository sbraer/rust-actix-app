use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

pub trait StructLike {}

pub trait RandomGenerator<T>
where
    T: StructLike,
{
    fn random() -> T;
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: &'static str,
    pub age: u8,
    pub date: NaiveDate,
}

impl StructLike for Person {}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {:?}", self.id, self.name, self.age, self.date)
    }
}

impl RandomGenerator<Person> for Person {
    fn random() -> Self {
        let names = ["Bob", "Ana", "Jessica", "Mike", "Rick"];
        let current_year = Local::now().year();
        let age: u8 = fastrand::u8(19..=99);
        let year_of_birth = current_year - i32::from(age);

        let i = fastrand::usize(..names.len());
        Self {
            id: fastrand::u32(1..=999),
            name: names[i],
            age,
            date: NaiveDate::from_ymd_opt(
                year_of_birth,
                fastrand::u32(1..=12),
                fastrand::u32(1..=28),
            )
            .unwrap_or_default(),
        }
    }
}
