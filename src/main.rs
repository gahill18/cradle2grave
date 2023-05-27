use rand::distributions::{Distribution, Standard, Uniform};
use rand::prelude::*;

type Name = String;
type Health = u8;
type Age = u8;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        todo!()
    }
}

#[derive(Debug)]
enum Sexuality {
    Homo,
    Hetero,
    Bi,
}
impl Distribution<Sexuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexuality {
        todo!("generate sexuality")
    }
}

#[derive(Debug)]
enum Trait {
    Brave,
    Coward,
    Lustful,
    Chaste,
}
impl Distribution<Trait> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Trait {
        todo!("generate traits")
    }
}

#[derive(Debug)]
struct Skills;
#[derive(Debug)]
enum Culture {
    Starbelly,
    Plainbelly,
}
impl Distribution<Culture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Culture {
        todo!("generate culture")
    }
}
#[derive(Debug)]
enum Religion {
    Boatism,
    Trainism,
}
impl Distribution<Religion> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Religion {
        todo!("generate religion")
    }
}
type Fame = i16;
type Opinion = i16;
type Stress = u8;
#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Arabic,
    Chinese,
}
impl Distribution<Language> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Language {
        todo!("generate language")
    }
}
#[derive(Debug)]
struct Education;

#[derive(Debug)]
struct Character {
    age: Age,
    culture: Culture,
    education: Education,
    fame: Fame,
    gender: Option<Gender>,
    health: Health,
    languages: Vec<Language>,
    name: Name,
    religion: Option<Religion>,
    sexuality: Option<Sexuality>,
    skills: Skills,
    stress: Stress,
    traits: Vec<Trait>,
}

impl Character {
    fn default() -> Self {
        let age: Age = 21;
        let culture: Culture = Culture::Starbelly;
        let fame: Fame = 0;
        let gender: Option<Gender> = Some(Gender::Male);
        let education: Education = Education;
        let health: Health = 10;
        let languages: Vec<Language> = vec![Language::English];
        let name: Name = String::from("Bob");
        let religion: Option<Religion> = Some(Religion::Boatism);
        let sexuality: Option<Sexuality> = Some(Sexuality::Hetero);
        let skills = Skills;
        let stress: Stress = 5;
        let traits: Vec<Trait> = vec![Trait::Brave];
        Character::new(
            age, culture, education, fame, gender, health, languages, name, religion, sexuality,
            skills, stress, traits,
        )
    }

    fn new(
        age: Age,
        culture: Culture,
        education: Education,
        fame: Fame,
        gender: Option<Gender>,
        health: Health,
        languages: Vec<Language>,
        name: Name,
        religion: Option<Religion>,
        sexuality: Option<Sexuality>,
        skills: Skills,
        stress: Stress,
        traits: Vec<Trait>,
    ) -> Self {
        Self {
            age,
            culture,
            education,
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

fn main() {
    println!("{:?}", Character::default());
}
