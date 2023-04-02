use anchor_lang::prelude::*;

declare_id!("AtpSWqNSWAJDt67VJWSJH62XRJARt35a3aJbskJBsMng");

#[program]
mod mercury_v1 {
    use super::*;

    // When user A connects his wallet
    // Maybe here we need to fetch the nft collection from user A
    pub fn init_user_a(ctx: Context<InitUserA>) -> Result<()> {
        let main_account = &mut ctx.accounts.main_account;
        let authority = &mut ctx.accounts.authority;

        main_account.authority = authority.key();

   
    Ok(())
    }

    // When user A introduces the user B wallet
    // Maybe here we should fetch the nft collection from user B
    pub fn wallet_user_b(ctx: Context<WalletUserB>) -> Result <()> {
        let account_user_b = &mut ctx.accounts.account_user_b;
        let authority = &mut ctx.accounts.authority;

        account_user_b.authority = authority.key();

    Ok(())
    }

    // When user A selects the user B NFT
    pub fn selected_nft_from_b(ctx: Context<SelectedNftFromB>) -> Result <()> {
        let nft_from_b = &mut ctx.accounts.nft_from_b;
        let authority = &mut ctx.accounts.authority;

        nft_from_b.authority = authority.key();

    Ok(())
    }

    // When user A selects his NFT
    pub fn selected_nft_from_a(ctx: Context<SelectedNftFromA>) -> Result <()> {
        let nft_from_a = &mut ctx.accounts.nft_from_a;
        let authority = &mut ctx.accounts.authority;

        nft_from_a.authority = authority.key();

    Ok(())
    }

    // When user A selects token and amount if is necessary
    pub fn token_amount(ctx: Context<TokenAmount>) -> Result <()> {
        let token_amount = &mut ctx.accounts.token_amount;
        let authority = &mut ctx.accounts.authority;

        token_amount = authority.key();

    Ok(())
    }

    // When user B connects his wallet
    pub fn init_user_b(ctx: Context<InitUserB>) -> Result <()> {
        let init_user_b = &mut ctx.accounts.init_user_b;
        let authority = &mut ctx.accounts.authority;

        inir_user_b = authority.key();

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
pub struct InitUserA<'info> {
    #[account(
        init,
        bump,
        payer = authority,
        space = 8 + 6264
        seeds =     // is it necessary??
    )]
    pub main_account: Account<'info, MainAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    
}

#[derive(Account)]
pub struct WalletUserB<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub user_b_account: Account<'info, AccountUserB>,
}

#[derive(Account)]
pub struct SelectedNftFromB<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub selected_nft_from_b: Account<'info, SelectedNftFromB>,
}


#[derive(Account)]
pub struct SelectedNftFromA<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub selected_nft_from_a: Account<'info, SelectedNftFromA>,
}

    #[derive(Account)]
pub struct TokenAmount<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub token_amount: Account<'info, TokenAmount>,
}

    #[derive(Account)]
pub struct InitUserB<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub init_user_b: Account<'info, InitUserB>,
}

    #[derive(Account)]
pub struct CheckAndDecide<'info> {
    #[account(
        init,
        bumb,
        payer = authority,
        space = 8 + 
        seeds =     // is it necessary??
    )]
    pub check_and_decide: Account<'info, CheckAndDecide>,
}


// -------------------------------------------------------------------
#[account]
pub struct MainAccount {
    pub authority: Pubkey,
    
}

#[account]
pub struct AccountUserB {
    pub authority: Pubkey,
}