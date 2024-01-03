use anchor_lang::prelude::*;

declare_id!("FpRALKURtearnyVigz6w1NVGQaHGZPGudR379gEomNfV");

#[program]
pub mod rust_chain {
    use super::*;
    
    pub fn initialize(ctx: Context<InitUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        msg!("User profile initialized");
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, post_id: Pubkey, title: String, date: String, content: String) -> Result<()> {
        let post = &mut ctx.accounts.post;
        post.post_id = post_id;
        post.title = title;
        post.date = date;
        post.content = content;
        Ok(())
    }
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>()
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
   #[account(mut)]
    pub authority: Signer<'info>, #[account(init,
        payer = authority,
        space = 8 + std::mem::size_of::<Post>()
    )]
    
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Post {
    pub post_id: Pubkey,
    pub title: String,
    pub date: String,
    pub content: String,
}

