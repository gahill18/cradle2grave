use crate::culture::Culture;
use crate::genders::Gender;
use crate::language::Language;
use crate::religion::Religion;
use crate::sexuality::Sexuality;
use crate::skills::Skills;
use crate::traits::Traits;

type Name = String;
type Age = u8;
type Health = u8;
type Fame = i16;
type Opinion = i16;
type Stress = u16;

#[derive(Debug)]
pub struct Character {
    age: Age,
    culture: Culture,
    fame: Fame,
    gender: Option<Gender>,
    health: Health,
    languages: Vec<Language>,
    name: Name,
    religion: Option<Religion>,
    sexuality: Option<Sexuality>,
    skills: Skills,
    stress: Stress,
    traits: Vec<Traits>,
}

impl Character {
    pub fn default() -> Self {
        let age: Age = 21;
        let culture: Culture = Culture::Starbelly;
        let fame: Fame = 0;
        let gender: Option<Gender> = Some(Gender::Male);
        let health: Health = 10;
        let languages: Vec<Language> = vec![Language::English];
        let name: Name = String::from("Bob");
        let religion: Option<Religion> = Some(Religion::Boatism);
        let sexuality: Option<Sexuality> = Some(Sexuality::Hetero);
        let skills = Skills::default();
        let stress: Stress = 5;
        let traits: Vec<Traits> = vec![Traits::brave()];
        Character::new(
            age, culture, fame, gender, health, languages, name, religion, sexuality, skills,
            stress, traits,
        )
    }

    fn new(
        age: Age,
        culture: Culture,
        fame: Fame,
        gender: Option<Gender>,
        health: Health,
        languages: Vec<Language>,
        name: Name,
        religion: Option<Religion>,
        sexuality: Option<Sexuality>,
        skills: Skills,
        stress: Stress,
        traits: Vec<Traits>,
    ) -> Self {
        Self {
            age,
            culture,
            fame,
            gender,
            health,
            languages,
            name,
            religion,
            sexuality,
            skills,
            stress,
            traits,
        }
    }
}
