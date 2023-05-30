use crate::characters::Character;

mod characters;
mod culture;
mod genders;
mod language;
mod religion;
mod sexuality;
mod skills;
mod traits;

fn main() {
    println!("{:?}", Character::default());
}
