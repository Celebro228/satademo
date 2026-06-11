use crate::ecs::*;
use crate::singleton::*;

pub struct StructCommands {

}

impl StructCommands {
    const fn new() -> Self {
        Self {

        }
    }
}

pub static COMMANDS: Single<StructCommands> = Single::new(StructCommands::new());