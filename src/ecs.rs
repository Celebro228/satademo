pub use flecs_ecs::{
    core::{Entity, SystemAPI, World as WD, flecs},
    macros::Component as CD
};
pub use inventory::*;
pub use satademo_macro::*;


pub struct SystemAdd(pub fn(&WD));
collect!(SystemAdd);


#[derive(CD)]
pub struct World {
    
}