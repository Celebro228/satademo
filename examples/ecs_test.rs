use satademo::prelude::*;

fn main() {
    let mut world = World::new();
    world.spawn((Dada, Lulu {}, ));

    for x in world.query_mut::<(&Dada,)>() {

    }
    for i in iter::<SystemUpdate> {
        i.0(&mut world);
    }

    satademo::run();
}

pub struct  Lulu {

}

pub struct Dada;


//#[component]
//pub struct Comps {
//    pub value: i32,
//}
//
//submit!(&'static Comps);

#[update]
fn ddd(c: &mut Dada) {
    println!("hello world!");
}


#[update]
fn dddd(c: &mut Dada) {
    println!("hello world!");
}