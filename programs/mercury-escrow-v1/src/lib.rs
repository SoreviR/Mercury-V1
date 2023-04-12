use anchor_lang::prelude::*;

declare_id!("AtpSWqNSWAJDt67VJWSJH62XRJARt35a3aJbskJBsMng");

#[constant]
pub const USER_SEED: &[u8] = b"user";

#[program]
mod mercury_v1 {
    use super::*;

    // When user A connects his wallet
    // Maybe here we need to fetch the nft collection from user A
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.authority = authority.key();

   
    Ok(())
    }

    // When user A introduces the user B wallet
    // I think here should it be all the parameters (user A address, user B address, nft A, nft B, token amount)
    // Maybe here we should fetch the nft collection from user B
    pub fn input_trade_info(ctx: Context<InputTradeInfo>, user_b_address: String, nft_a: String, nft_b: String, token: String, token_amount: u16) -> Result <()> {
        let trade_info_account = &mut ctx.accounts.trade_info_account; // check?
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        trade_info_account.authority = authority.key();  // check?
        
    Ok(())
    }

    // When user B connects his wallet
    pub fn init_user_b(ctx: Context<InitUserB>) -> Result <()> {
        let init_user_b = &mut ctx.accounts.init_user_b;
        let authority = &mut ctx.accounts.authority;

        init_user_b = authority.key();

    Ok(())
    }

    // When user B fetchs the transaction created by user A and decide to accept or cancel
    pub fn check_and_decide(ctx: Context<CheckAndDecide>) -> Result<()> {

        let check_and_decide = &mut ctx.accounts.check_and_decide;
        let authority = &mut ctx.accounts.authority;

        check_and_decide = authority.key();

    Ok(())
    }
 }


// We need a cancel function
// We need to add a timer

// --------------------------------------------------------------------------
#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + ,
        
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
}

#[derive(Accounts)]
pub struct InputTradeInfo<'info> {
    #[account(
        init,
        seeds = [],     // is it necessary??,
        bump,
        payer = authority,
        space = 8 + ,
                
    )]
    pub trade_info_account: Account<'info, TradeInfoAccount>,
}

// #[derive(Account)]
// pub struct SelectedNftFromB<'info> {
//     #[account(
//         init,
//         bump,
//         payer = authority,
//         space = 8 + 
//         seeds =     // is it necessary??
//     )]
//     pub selected_nft_from_b: Account<'info, SelectedNftFromB>,


// #[derive(Account)]
// pub struct SelectedNftFromA<'info> {
//     #[account(
//         init,
//         bump,
//         payer = authority,
//         space = 8 + 
//         seeds =     // is it necessary??
//     )]
//     pub selected_nft_from_a: Account<'info, SelectedNftFromA>,

// #[derive(Account)]
// pub struct TokenAmount<'info> {
//     #[account(
//         init,
//         bump,
//         payer = authority,
//         space = 8 + 
//         seeds =     // is it necessary??
//     )]
//     pub token_amount: Account<'info, TokenAmount>,

#[derive(Accounts)]
pub struct InitUserB<'info> {
    #[account(
        init,
        seeds = [],    // is it necessary??
        bump,
        payer = authority,
        space = 8 + ,        
    )]
    pub init_user_b: Account<'info, InitUserB>,
}

#[derive(Accounts)]
pub struct CheckAndDecide<'info> {
    #[account(
        init,
        seeds = [],    // is it necessary??
        bump,
        payer = authority,
        space = 8 + ,        
    )]
    pub check_and_decide: Account<'info, CheckAndDecide>,
}


// -------------------------------------------------------------------
#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    
}

#[account]
pub struct TradeInfoAccount {
    pub user_a_address: Pubkey,
    pub user_b_address: String,  // should it be Pubkey as well or is the address from user B that user A write??
    pub nft_a: String,    // should it be a string or is there another thing?? 
    pub nft_b: String,    // should it be a string or is there another thing??
    pub token: String,    // should it be a string or is there another thing??
    pub token_amount: u16,
    pub authority: Pubkey,
}
