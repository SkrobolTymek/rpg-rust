use std::io;

struct Player {
    health: i32,
    strength: i32,
    gold: i32,
}
struct Monster {
    health: i32;
    strength: i32;
}



fn main() {
    let saizzan = Player {
        health: 500,
    strength: 500,
    gold: 500,
    };
    let tralalelo_tralala = Monster{
        health: 10,
        strength: 15,
    };
    let monster_dmg = tralalelo_tralala.strength * 2;
    let dmg = saizzan.strength * 2;
    let wybor = String::new;
    println!("dmg saizzana boga wynosi: {}", dmg);
    let is_alive = if saizzan.health <= 0{
        false
    }
    else{
true
    };
    println!("{}", is_alive);
}

