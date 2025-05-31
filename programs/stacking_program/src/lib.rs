pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CS3afP5HKjUkUCifygoRg57ecdzUqaNiGvgRgzwtzePP");

#[program]
pub mod stacking_program {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        todo!()
    }
}
