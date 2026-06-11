use satademo::prelude::*;
use satademo::singleton;

fn main() {
    let mut world = World::new();
    world.spawn((Dada(0), Lulu {}, ));
    let dur = std::time::Instant::now();
    for i in iter::<SystemUpdate> {
        i.0(&mut world);
    }
    dbg!(dur.elapsed());
    println!("size: {}", world.query::<(&Dada,)>().iter().collect::<Vec<(&Dada,)>>().len());
    for (i) in &mut world.query::<(&Dada,)>() {
        println!("{}", i.0.0);
        break;
    }

    satademo::run();
    DATA.set(1);
    println!("{}", *DATA);
}


singleton!(DATA, usize, 1);

pub struct  Lulu {

}

pub struct Dada(pub usize);


//#[component]
//pub struct Comps {
//    pub value: i32,
//}
//
//submit!(&'static Comps);

#[update]
fn ddd() {

}


#[start]
fn dddd(c: &mut Dada) {}

//function!(1000);