use anchor_lang::prelude::*;

declare_id!("338BZQTraQYe4yLfDa6Cb4WMACeF3ccv3umBKTER7R96");

#[program]
pub mod gifportalbackend {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvoted_users: Vec::new(),
            downvoted_users: Vec::new(),
            vote_count: 0,
        };
            
        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif_list = &mut base_account.gif_list;

        for item in gif_list {
            if item.gif_link == gif_link {
                item.upvoted_users.push(*user.to_account_info().key);
                item.vote_count += 1;
            }
        }
        Ok(())
    }

    pub fn cancle_upvote_gif(ctx: Context<CancleUpvoteGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif_list = &mut base_account.gif_list;

        for item in gif_list {
            if item.gif_link == gif_link {
                let mut index: usize = 0;
                let upvoted_users_clone = item.upvoted_users.clone();
                for upvoted_user in upvoted_users_clone {
                    if upvoted_user == *user.to_account_info().key {
                        item.upvoted_users.remove(index);
                        item.vote_count -= 1;
                    }
                    index += 1;
                }
            }
        }
        Ok(())
    }

    pub fn downvote_gif(ctx: Context<DownvoteGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif_list = &mut base_account.gif_list;

        for item in gif_list {
            if item.gif_link == gif_link {
                item.downvoted_users.push(*user.to_account_info().key);
                item.vote_count -= 1;
            }
        }
        Ok(())
    }

    pub fn cancle_downvote_gif(ctx: Context<CancleDownvoteGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif_list = &mut base_account.gif_list;

        for item in gif_list {
            if item.gif_link == gif_link {
                let mut index: usize = 0;
                let downvoted_users_clone = item.downvoted_users.clone();
                for downvoted_user in downvoted_users_clone {
                    if downvoted_user == *user.to_account_info().key {
                        item.downvoted_users.remove(index);
                        item.vote_count += 1;
                    }
                    index += 1;
                }
            }
        }
        Ok(())
    }

    pub fn remove_gif(ctx: Context<RemoveGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        // Clone gif_list to use it as an iterator as a multi-use would cause a borrowd move error
        let cloned_gif_list = base_account.gif_list.clone();
        let mut index: usize = 0;

        for item in cloned_gif_list {
            if item.gif_link == gif_link && item.user_address == *user.to_account_info().key {
                base_account.gif_list.remove(index);
            }
            index += 1;
        }

        Ok(())
    }

}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // init will tell Solana to create a new account owned by our current program.
    // payer = user tells our program who's paying for the account to be created. 
    //      In this case, it's the user calling the function.
    // We then say space = 9000 which will allocate 9000 bytes of space for our account. 
    //      You can change this # if you wanted, but, 9000 bytes is enough for the program we'll be building here!
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    // Allow access to a mutable reference
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
#[derive(Accounts)]
pub struct AddGif<'info> {
    // Allow access to a mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
    // Allow access to a mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancleUpvoteGif<'info> {
    // Allow access to a mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DownvoteGif<'info> {
    // Allow access to a mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancleDownvoteGif<'info> {
    // Allow access to a mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Specify what data you want in the RemoveGif Context.
#[derive(Accounts)]
pub struct RemoveGif<'info> {
    // Allow access to mutable reference
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with. The AnchorSerialize and AnchorDeserialize attributes
// show anchor how to serialize and desearialize data
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvoted_users: Vec<Pubkey>,
    pub downvoted_users: Vec<Pubkey>,
    pub vote_count: i32,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
