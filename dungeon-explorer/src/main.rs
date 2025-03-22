use std::io;
use rand::Rng;

struct Player {
    health: i32,
    strength: i32,
    gold: i32,
    regeneration: i32,
    max_health: i32,
    elixirs: i32,
}

#[derive(Clone)]
struct Monster {
    health: i32,
    strength: i32,
    name: String,
}

fn main() {
    let mut saizzan = Player {
        health: 100,
        strength: 15,
        gold: 600,
        regeneration: 10,
        max_health: 100,
        elixirs: 0, 
    };

    let monsters = vec![
        Monster { health: 40, strength: 15, name: "tralalelo_tralala".to_string() },  
        Monster { health: 60, strength: 20, name: "bombardino_crocodilo".to_string() },  
        Monster { health: 100, strength: 30, name: "dildodini_penisini".to_string() }, 
        Monster { health: 80, strength: 25, name: "chrumbo_grande".to_string() },  
        Monster { health: 200, strength: 50, name: "gigachadus_maximus".to_string() },
    ];

    let mut rng = rand::thread_rng();

    while saizzan.health > 0 {
        let random_index = rng.gen_range(0..monsters.len());
        let mut chosen_monster = monsters[random_index].clone();


        println!("=============================");
        println!("NAPOTKALES POTWORA: {}", chosen_monster.name);
        println!("HP: {} | Siła: {}", chosen_monster.health, chosen_monster.strength);
        println!("=============================");

        while chosen_monster.health > 0 {
            println!("1 - Atak");
            println!("2 - Ulecz się");
            println!("3 - Kup leczace elixiry (15 golda za jeden)");

            let mut wybor = String::new();
            io::stdin().read_line(&mut wybor).expect("Blad zczytywania");
            let wybor = wybor.trim();

            match wybor {
                "1" => {
                    chosen_monster.health -= saizzan.strength * 2;
                    println!("ZAATAKOWAŁEŚ POTWORA, JEGO HP TERAZ TO: {}", chosen_monster.health);

                    if chosen_monster.health <= 0 {
                        println!("wow, pokonales potwora, zdobywasz zloto.");
                        saizzan.gold += 30;
                        break;  
                    } else {
                        println!("Potwór atakuje!");
                        saizzan.health -= chosen_monster.strength;
                        println!("Twoje HP: {}", saizzan.health);

                        if saizzan.health <= 0 {
                            println!("Dildodini penisini, tralalelo tralala, umarłeś.");
                            return;
                        }
                    }
                }
                "2" => {
                    if saizzan.health == saizzan.max_health {
                        println!("Masz już pelne HP!");
                    } if saizzan.elixirs > 0 {
                        saizzan.health = (saizzan.health + saizzan.regeneration).min(saizzan.max_health);
                        saizzan.elixirs -= 1;
                        println!("Masz teraz tyle HP: {}", saizzan.health);
                    }
                    else{
                        println!("Nie masz elixirow");
                    }
                }
                "3" =>{
                    if saizzan.gold > 15{
                        println!("Kupiles elixir");
                        saizzan.elixirs +=1;
                    }
                    else{
                        println!("Nie stac cie XDDD");
                    }
                }
                "9" =>{
                    std::process::exit(0)
                }
                _ => {
                    println!("Wybrałeś nieistniejącą opcję");
                }
            }
        }
    }
}
