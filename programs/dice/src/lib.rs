use anchor_lang::prelude::*;

declare_id!("B3eJtrwzFYYT4ifWkro3RPNwwbXbgaZpDRGbvwAfmqzo");

pub mod contexts;
pub mod state;

#[program]
pub mod dice {
    use super::*;

}

#[derive(Accounts)]
pub struct Initialize {}
