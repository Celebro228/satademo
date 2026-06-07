use satademo::prelude::*;

fn main() {
    let world = flecs_ecs::core::World::new();
    world.entity().set(Comps { value: 42 });
    for i in inventory::iter::<SystemAdd> {
        i.0(&world);
    }
    world.progress();
    let c: [&dyn Lulu; 1] = [&Dada];
    let c: [&mut dyn Lulu; 1] = [&mut Dada];
}

pub trait Lulu {
    
}

pub struct Dada;

impl Lulu for Dada {
    
}

#[component]
pub struct Comps {
    pub value: i32,
}

#[update]
fn ddd(c: &mut Comps) {
    println!("hello world!");
}