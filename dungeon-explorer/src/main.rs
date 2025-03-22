use std::io;
use rand::Rng;

#[derive(Clone)]
struct Player {
    health: i32,
    strength: i32,
    gold: i32,
    regeneration: i32,
    max_health: i32,
    elixirs: i32,
    ekwipunek: Vec<String>, 
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
        ekwipunek: Vec::new(),  
    };

    let monsters = vec![
        Monster { health: 40, strength: 15, name: "tralalelo_tralala".to_string() },
        Monster { health: 60, strength: 20, name: "bombardino_crocodilo".to_string() },
        Monster { health: 100, strength: 30, name: "dildodini_penisini".to_string() },
        Monster { health: 80, strength: 25, name: "chrumbo_grande".to_string() },
        Monster { health: 200, strength: 50, name: "gigachadus_maximus".to_string() },
    ];

    let nagrody = vec![
        "Krysztalowa kula - instakill przeciwnika",
        "Jabol Smocza Krew – Regeneruje pełne HP",
        "Eliksir Czteropaka – Podwaja twoją siłę",
        "Zbutwiały Sandał – Potwór traci turę, bo musi przemyśleć swoje życiowe wybory.",
        "Łzawa Poezja Emo – Potwór dostaje depresji i popełnia samobójstwo.",
    ];

    let mut rng = rand::thread_rng();
    let mut traci_ture = false;

    while saizzan.health > 0 {
        let random_index = rng.gen_range(0..monsters.len());
        let mut chosen_monster = monsters[random_index].clone();
        let random_index_items = rng.gen_range(0..nagrody.len());
        let zdobyta_nagroda = nagrody[random_index_items].to_string();

        println!("=============================");
        println!("NAPOTKALES POTWORA: {}", chosen_monster.name);
        println!("HP: {} | Siła: {}", chosen_monster.health, chosen_monster.strength);
        println!("=============================");

        while chosen_monster.health > 0 {
            println!("1 - Atak");
            println!("2 - Ulecz się");
            println!("3 - Kup leczace elixiry (15 golda za jeden)");
            println!("4 - pokaz ekwipunek/uzyj przedmiotu");

            let mut wybor = String::new();
            io::stdin().read_line(&mut wybor).expect("Blad zczytywania");
            let wybor = wybor.trim();
            
            match wybor {
                "1" => {
                    chosen_monster.health -= saizzan.strength * 2;
                    println!("ZAATAKOWAŁEŚ POTWORA, JEGO HP TERAZ TO: {}", chosen_monster.health);

                    if chosen_monster.health <= 0 {
                        println!("Pokonałeś potwora, zdobywasz nagrodę!");
                        match chosen_monster.name.as_str() {
                            "tralalelo_tralala" => {
                                saizzan.gold += 15;
                                println!("Otrzymałeś 15 złota");
                            }
                            "bombardino_crocodilo" => {
                                saizzan.gold += 30;
                                println!("Otrzymałeś 30 złota");
                            }
                            "dildodini_penisini" => {
                                saizzan.gold += 50;
                                println!("Otrzymałeś 50 złota");
                            }
                            "chrumbo_grande" => {
                                saizzan.gold += 75;
                                println!("Otrzymałeś 75 złota");
                            }
                            "gigachadus_maximus" => {
                                saizzan.gold += 200;
                                println!("Otrzymałeś 200 złota");
                            }
                            _ => {}
                        }
                        
                    
                        saizzan.ekwipunek.push(zdobyta_nagroda.clone());
                        println!("ZNALAZLES: {}", zdobyta_nagroda);
                        break;
                    } else {
                        if traci_ture ==false{
                        println!("Potwór atakuje!");
                        saizzan.health -= chosen_monster.strength;
                        println!("Twoje HP: {}", saizzan.health);
                        if saizzan.health <= 0 {
                            println!("Dildodini penisini, tralalelo tralala, umarłeś.");
                            return;
                        }
                    }
                    else {
                        println!("Potwor nie ma tury");
                        traci_ture = false;
                    }
                       
                    }
                }
                "2" => {
                    if saizzan.health == saizzan.max_health {
                        println!("Masz już pelne HP!");
                    } else if saizzan.elixirs > 0 {
                        saizzan.health = (saizzan.health + saizzan.regeneration).min(saizzan.max_health);
                        saizzan.elixirs -= 1;
                        println!("Masz teraz tyle HP: {}", saizzan.health);
                    } else {
                        println!("Nie masz eliksirów");
                    }
                }
                "3" => {
                    if saizzan.gold >= 15 {
                        println!("Kupiles elixir");
                        saizzan.elixirs += 1;
                        saizzan.gold -= 15;
                    } else {
                        println!("Nie stać Cię XDDD");
                    }
                }
                "4" => {
                    if saizzan.ekwipunek.is_empty() {
                        println!("Twój ekwipunek jest pusty.");
                    } else {
                        println!("Twój ekwipunek:");
                        for (i, item) in saizzan.ekwipunek.iter().enumerate() {
                            println!("{} - {}", i + 1, item);
                        }

                        let mut item_choice = String::new();
                        io::stdin().read_line(&mut item_choice).expect("Błąd zczytywania");
                        let item_choice: usize = item_choice.trim().parse().expect("Wybierz numer przedmiotu");

                        if item_choice > 0 && item_choice <= saizzan.ekwipunek.len() {
                            let item = saizzan.ekwipunek.remove(item_choice - 1);

                            match item.as_str() {
                                "Jabol Smocza Krew – Regeneruje pełne HP" => {
                                    saizzan.health = saizzan.max_health;
                                    println!("Użyłeś Jabol Smocza Krew, pełne HP!");
                                }
                                "Eliksir Czteropaka – Podwaja twoją siłę" => {
                                    saizzan.strength *= 2;
                                    println!("Użyłeś Eliksiru Czteropaka, twoja siła została podwojona!");
                                }
                                "Zbutwiały Sandał – Potwór traci turę" => {
                                    println!("Użyłeś Zbutwiały Sandał, potwór traci turę!");
                                    traci_ture = true;
                                }
                                "Łzawa Poezja Emo – Potwór dostaje depresji" => {
                                    println!("Użyłeś Łzawą Poezję Emo, potwór popełnia samobójstwo!");
                                    chosen_monster.health = 0;
                                }
                                "Krysztalowa kula - instakill przeciwnika" => {
                                    println!("Użyłeś Krysztalową kulę, potwór umiera natychmiast!");
                                    chosen_monster.health = 0;
                                }
                                _ => {
                                    println!("Nieznany przedmiot");
                                }
                            }
                        } else {
                            println!("Zły wybór przedmiotu");
                        }
                    }
                }
                "9" => {
                    std::process::exit(0);
                }
                _ => {
                    println!("Wybrałeś nieistniejącą opcję");
                }
            }
        }
    }
}
