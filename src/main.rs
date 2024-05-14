use std::fmt::Debug;

struct Monster {
    health: i32,

}

#[derive(Debug)]
struct Wizard {
    health: i32,
    mana: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {}
trait Melee {}
trait Ranged {}

impl Melee for Ranger {}
impl Melee for Wizard {}
impl Ranged for Ranger {}
impl Magic for Wizard {}

fn attack_with_bow <T:Melee+Debug> (character:&T, opponent:&mut Monster, distance:i32) {
    if distance < 10 {
        opponent.health -= 10;
        println!("{:?} attacked with bow, monster health: {}", character, opponent.health);
    }
}

fn attack_with_sword <T:Melee+Debug> (character:&T, opponent:&mut Monster) {
    opponent.health -= 20;
    println!("{:?} attacked with sword, monster health: {}", character, opponent.health);
}

fn fireball <T:Magic+Debug> (character:&T, opponent:&mut Monster, distance:i32) {
    if distance < 15 {
        opponent.health -= 30;
        println!("{:?} attacked with fireball, monster health: {}", character, opponent.health);
    }
}

fn main() {
    let mut monster = Monster { health: 100 };
    let wizard = Wizard { health: 100, mana: 100 };
    let ranger = Ranger { health: 100 };

    attack_with_bow(&ranger, &mut monster, 5);
    attack_with_sword(&ranger, &mut monster);
    fireball(&wizard, &mut monster, 10);
}
