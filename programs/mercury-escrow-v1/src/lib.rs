///Brief meeting summary:

///- Currently trying to do too much at once. Get the MVP of NFT for NFT swaps done first, then add the other stuff later once it's working
///- There should be 3 functions, create, accept and cancel.
///- Use contexts and accounts instead of passing in strings
///- Follow the NFT transfer code that I sent to you
///- Save the mint_a, mint_b and from_token_account and signer from the create function to use in the cancel/accept functions

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};

declare_id!("7tLrzaraBbJgRqFHpwCKiwhv7pZoE9xMKwDkjKU4sxJQ");


// ------------------------------------ INSTRUCTIONS ---------------------------------------------------------


#[program]
mod mercury_v1 {
    use super::*;

// Create basket function: to create the PDA account (PDA)
// the PDA is both the address of the Token Account and the authority of the token account
    pub fn create_basket(ctx: Context<CreateBasket>) -> Result<()> {

        // Create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(), // should be payer,  authority or auth? from context struct
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        token::mint_to(cpi_ctx, 1)?;
        
        Ok(());
        
    }

    
// Accept function:
    pub fn accept_swap(ctx:Context<TokenTransfer>) -> Result<()> {
        let seeds = &["auth".as_bytes(), &[*ctx.bumps.get("auth").unwrap()]];
        let signer = [&seeds[..]];
    
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.auth.to_account_info(),
            },
            &signer,
        );
    
        token::transfer(cpi_ctx, 1)?;
        Ok(());
    }


// Cancel function:
    pub fn cancel_swap(ctx:Context<CancelSwap>) -> Result <()> {


        Ok(());
    }
}

// -------------------------------------- CONTEXT ----------------------------------------

#[derive(Accounts)]
pub struct CreateBasket<'info> {
   #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = auth,
        seeds = ["auth".as_bytes().as_ref()],
        bump,
    )]
    pub token_account: Account<'info, TokenAccount>,  
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: PDA
    #[account(
        seeds = ["auth".as_bytes().as_ref()],
        bump,
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct TokenTransfer<'info> {
    /// CHECK: token account authority PDA
    #[account(
        seeds = ["auth".as_bytes().as_ref()],
        bump,
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = auth
    )]
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CancelSwap<'info> {
    #[account(
        seeds = ["auth".as_bytes().as_ref()],
        bump,
    )]
}









// -------------------------------------- ACCOUNTS (States)---------------------------------------- 

// #[account]
// pub struct BasketAccount {
//     pub user_a_address: Pubkey,
//     pub user_b_address: Pubkey,
//     pub nft_a: ,
//     pub nft_b: ,
//     pub authority: Pubkey,
// }