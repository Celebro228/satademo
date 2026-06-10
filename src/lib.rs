pub mod prelude;
pub mod ecs;

pub fn run() {
    let mut world = ecs::World::new();

    ecs::system_start_run(&mut world);
    for _ in 0..100 {
        ecs::system_update_run(&mut world);
    }
    ecs::system_exit_run(&mut world);
}