use anchor_lang::prelude::*;

declare_id!("52cGns3aQ3UYrbb7FTztaJXhxmbHku1TCVi4CaLsVszg");

#[program]
pub mod speedrun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
