use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use fake::faker::name::en::Name;
use fake::Fake;

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub date: NaiveDate,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {:?}", self.id, self.name, self.age, self.date)
    }
}

impl Person {
    pub fn random() -> Self {
        let current_year = Local::now().year();
        let age: u8 = (18..=99).fake::<u8>();
        let year_of_birth = current_year - i32::from(age);

        Self {
            id: (1..1000).fake::<u32>(),
            name: Name().fake(),
            age,
            date: NaiveDate::from_ymd_opt(
                year_of_birth,
                (1..=12).fake::<u32>(),
                (1..=28).fake::<u32>(),
            ).unwrap_or_default(),
        }
    }
}

pub fn create_person_collection() -> Vec<Person> {
    vec![
        Person {
            id: 1,
            name: "Mario".to_string(),
            age: 43,
            date: NaiveDate::from_ymd_opt(1981, 2, 21).unwrap(),
        },
        Person {
            id: 2,
            name: "Luigi".to_string(),
            age: 41,
            date: NaiveDate::from_ymd_opt(1983, 3, 25).unwrap(),
        },
    ]
}
