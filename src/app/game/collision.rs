use rand::rngs::SmallRng;

use super::{Enemy, Item, Player, item::Variant};

pub fn check(rng: &mut SmallRng, player: &mut Player, item: &mut Item, enemies: &Vec<Enemy>) {
    item_check(rng, item, player);
    enemy_check(enemies, player);
}

fn item_check(rng: &mut SmallRng, item: &mut Item, player: &mut Player) {
    let x = (item.pos().x - player.pos().x).abs();
    let y = (item.pos().y - player.pos().y).abs();

    if x < 2.0 && y < 1.0 {
        match item.variant() {
            Variant::ScoreUp => {
                player.add_score();
            }
            Variant::HealthUp => {
                player.add_health();
            }
        }

        item.reposition(rng);
    }
}

fn enemy_check(enemies: &Vec<Enemy>, player: &mut Player) {
    for enemy in enemies {
        let x = (enemy.pos().x - player.pos().x).abs();
        let y = (enemy.pos().y - player.pos().y).abs();

        if x < 1.0 && y < 1.0 {
            player.take_damage();
        }
    }
}
