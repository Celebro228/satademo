pub use inventory::*;
pub use satademo_macro::*;
pub use hecs::*;

collect!(SystemStart);
collect!(SystemUpdate);
collect!(SystemExit);

pub struct SystemStart(pub fn(&mut World));
pub struct SystemUpdate(pub fn(&mut World));
pub struct SystemExit(pub fn(&mut World));

#[inline(always)]
pub(crate) fn system_start_run(world: &mut World) {
    for system in iter::<SystemStart>() {
        system.0(world);
    }
}

#[inline(always)]
pub(crate) fn system_update_run(world: &mut World) {
    for system in iter::<SystemUpdate>() {
        system.0(world);
    }
}

#[inline(always)]
pub(crate) fn system_exit_run(world: &mut World) {
    for system in iter::<SystemExit>() {
        system.0(world);
    }
}