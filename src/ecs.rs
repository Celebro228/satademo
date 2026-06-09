pub use inventory::*;
pub use satademo_macro::*;
pub use hecs::*;

collect!(SystemStart);
collect!(SystemUpdate);
collect!(SystemExit);

pub struct SystemStart(pub fn(&mut World));
pub struct SystemUpdate(pub fn(&mut World));
pub struct SystemExit(pub fn(&mut World));