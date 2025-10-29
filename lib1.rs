use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111112");

#[program]
pub mod autonomous_vehicle_payments {
    use super::*;

    // authority: Address, System administrator, 9PJ8I...3555
    // fee_bps: Number, Platform fee percentage, 250 = 2.5%
    // treasury: Address, Fee collection address, 8KL9M...4444
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        fee_bps: u16,
        treasury: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.bump = ctx.bumps.config;
        config.authority = ctx.accounts.authority.key();
        config.is_active = true;
        config.is_paused = false;
        config.fee_bps = fee_bps;
        config.treasury = treasury;
        config.version = 1;
        Ok(())
    }

    // vehicle_id: String, Unique vehicle identifier, "AV-001"
    // operator: Address, Vehicle operator wallet, 7GH8J...2222
    // location: String, Current location coords, "40.7128,-74.0060"
    pub fn register_vehicle(
        ctx: Context<RegisterVehicle>,
        vehicle_id: String,
        operator: Pubkey,
        location: String,
    ) -> Result<()> {
        require!(vehicle_id.len() <= 32, ErrorCode::InvalidParameter);
        require!(location.len() <= 64, ErrorCode::InvalidParameter);

        let config = &ctx.accounts.config;
        require!(config.is_active && !config.is_paused, ErrorCode::ConfigInactive);

        let vehicle = &mut ctx.accounts.vehicle;
        vehicle.bump = ctx.bumps.vehicle;
        vehicle.vehicle_id = vehicle_id;
        vehicle.operator = operator;
        vehicle.location = location;
        vehicle.is_active = true;
        vehicle.is_busy = false;
        vehicle.total_deliveries = 0;
        vehicle.registered_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // delivery_id: Number, Unique delivery identifier, 12345
    // payment_amount: Number, Payment in lamports, 1000000000 = 1 SOL
    // pickup_location: String, Pickup coordinates, "40.7128,-74.0060"
    // delivery_location: String, Delivery coordinates, "40.7589,-73.9851"
    pub fn create_delivery_order(
        ctx: Context<CreateDeliveryOrder>,
        delivery_id: u64,
        payment_amount: u64,
        pickup_location: String,
        delivery_location: String,
    ) -> Result<()> {
        require!(pickup_location.len() <= 64, ErrorCode::InvalidParameter);
        require!(delivery_location.len() <= 64, ErrorCode::InvalidParameter);
        require!(payment_amount > 0, ErrorCode::InvalidAmount);

        let config = &ctx.accounts.config;
        require!(config.is_active && !config.is_paused, ErrorCode::ConfigInactive);

        let customer_key = ctx.accounts.customer.key();

        // Escrow payment from customer
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.customer.to_account_info(),
                    to: ctx.accounts.escrow.to_account_info(),
                },
            ),
            payment_amount,
        )?;

        let delivery = &mut ctx.accounts.delivery;
        delivery.bump = ctx.bumps.delivery;
        delivery.delivery_id = delivery_id;
        delivery.customer = customer_key;
        delivery.payment_amount = payment_amount;
        delivery.pickup_location = pickup_location;
        delivery.delivery_location = delivery_location;
        delivery.status = DeliveryStatus::Pending;
        delivery.assigned_vehicle = None;
        delivery.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // delivery_id: Number, Target delivery order, 12345
    pub fn accept_delivery(ctx: Context<AcceptDelivery>, delivery_id: u64) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(config.is_active && !config.is_paused, ErrorCode::ConfigInactive);

        let vehicle = &ctx.accounts.vehicle;
        require!(vehicle.is_active && !vehicle.is_busy, ErrorCode::VehicleNotAvailable);

        let delivery = &mut ctx.accounts.delivery;
        require!(delivery.status == DeliveryStatus::Pending, ErrorCode::InvalidDeliveryStatus);

        let vehicle_mut = &mut ctx.accounts.vehicle;
        vehicle_mut.is_busy = true;

        delivery.status = DeliveryStatus::InProgress;
        delivery.assigned_vehicle = Some(ctx.accounts.vehicle.key());
        delivery.accepted_at = Some(Clock::get()?.unix_timestamp);
        Ok(())
    }

    // delivery_id: Number, Completed delivery order, 12345
    pub fn complete_delivery(ctx: Context<CompleteDelivery>, delivery_id: u64) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(config.is_active && !config.is_paused, ErrorCode::ConfigInactive);

        let delivery = &ctx.accounts.delivery;
        require!(delivery.status == DeliveryStatus::InProgress, ErrorCode::InvalidDeliveryStatus);
        require!(
            delivery.assigned_vehicle == Some(ctx.accounts.vehicle.key()),
            ErrorCode::Unauthorized
        );

        let customer_key = ctx.accounts.customer.key();
        let vehicle_key = ctx.accounts.vehicle.key();
        let config_key = ctx.accounts.config.key();

        // Calculate fee and payment
        let fee = delivery.payment_amount
            .checked_mul(config.fee_bps as u64)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_div(10000)
            .ok_or(ErrorCode::MathOverflow)?;
        let vehicle_payment = delivery.payment_amount
            .checked_sub(fee)
            .ok_or(ErrorCode::MathOverflow)?;

        // Transfer payment to vehicle operator
        let escrow_bump = [ctx.bumps.escrow];
        let escrow_seeds = &[
            b"escrow",
            customer_key.as_ref(),
            &delivery_id.to_le_bytes(),
            &escrow_bump
        ];
        let signer_seeds: &[&[&[u8]]] = &[escrow_seeds];

        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= vehicle_payment;
        **ctx.accounts.vehicle_operator.to_account_info().try_borrow_mut_lamports()? += vehicle_payment;

        // Transfer fee to treasury
        **ctx.accounts.escrow.to_account_info().try_borrow_mut_lamports()? -= fee;
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += fee;

        let delivery_mut = &mut ctx.accounts.delivery;
        delivery_mut.status = DeliveryStatus::Completed;
        delivery_mut.completed_at = Some(Clock::get()?.unix_timestamp);

        let vehicle_mut = &mut ctx.accounts.vehicle;
        vehicle_mut.is_busy = false;
        vehicle_mut.total_deliveries = vehicle_mut.total_deliveries
            .checked_add(1)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        seeds = [b"config", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + Config::LEN
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(vehicle_id: String)]
pub struct RegisterVehicle<'info> {
    #[account(
        init,
        seeds = [b"vehicle", vehicle_id.as_bytes()],
        bump,
        payer = authority,
        space = 8 + Vehicle::LEN
    )]
    pub vehicle: Account<'info, Vehicle>,
    #[account(
        seeds = [b"config", authority.key().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(delivery_id: u64)]
pub struct CreateDeliveryOrder<'info> {
    #[account(
        init,
        seeds = [b"delivery", customer.key().as_ref(), &delivery_id.to_le_bytes()],
        bump,
        payer = customer,
        space = 8 + Delivery::LEN
    )]
    pub delivery: Account<'info, Delivery>,
    #[account(
        init,
        seeds = [b"escrow", customer.key().as_ref(), &delivery_id.to_le_bytes()],
        bump,
        payer = customer,
        space = 0
    )]
    /// CHECK: PDA for holding escrowed payment
    pub escrow: AccountInfo<'info>,
    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub customer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(delivery_id: u64)]
pub struct AcceptDelivery<'info> {
    #[account(
        mut,
        seeds = [b"delivery", delivery.customer.as_ref(), &delivery_id.to_le_bytes()],
        bump = delivery.bump,
    )]
    pub delivery: Account<'info, Delivery>,
    #[account(
        mut,
        seeds = [b"vehicle", vehicle.vehicle_id.as_bytes()],
        bump = vehicle.bump,
    )]
    pub vehicle: Account<'info, Vehicle>,
    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    pub operator: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(delivery_id: u64)]
pub struct CompleteDelivery<'info> {
    #[account(
        mut,
        seeds = [b"delivery", customer.key().as_ref(), &delivery_id.to_le_bytes()],
        bump = delivery.bump,
    )]
    pub delivery: Account<'info, Delivery>,
    #[account(
        mut,
        seeds = [b"escrow", customer.key().as_ref(), &delivery_id.to_le_bytes()],
        bump,
    )]
    /// CHECK: PDA holding escrowed payment
    pub escrow: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"vehicle", vehicle.vehicle_id.as_bytes()],
        bump = vehicle.bump,
    )]
    pub vehicle: Account<'info, Vehicle>,
    /// CHECK: Vehicle operator receiving payment
    #[account(mut)]
    pub vehicle_operator: AccountInfo<'info>,
    /// CHECK: Verified through config.treasury constraint
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(
        seeds = [b"config", config.authority.as_ref()],
        bump = config.bump,
        constraint = treasury.key() == config.treasury @ ErrorCode::InvalidTreasury
    )]
    pub config: Account<'info, Config>,
    /// CHECK: Customer account for seed derivation
    pub customer: AccountInfo<'info>,
}

#[account]
pub struct Config {
    pub bump: u8,
    pub authority: Pubkey,
    pub is_active: bool,
    pub is_paused: bool,
    pub fee_bps: u16,
    pub treasury: Pubkey,
    pub version: u8,
}
impl Config { pub const LEN: usize = 1 + 32 + 1 + 1 + 2 + 32 + 1; }

#[account]
pub struct Vehicle {
    pub bump: u8,
    pub vehicle_id: String,
    pub operator: Pubkey,
    pub location: String,
    pub is_active: bool,
    pub is_busy: bool,
    pub total_deliveries: u64,
    pub registered_at: i64,
}
impl Vehicle { pub const LEN: usize = 1 + (4 + 32) + 32 + (4 + 64) + 1 + 1 + 8 + 8; }

#[account]
pub struct Delivery {
    pub bump: u8,
    pub delivery_id: u64,
    pub customer: Pubkey,
    pub payment_amount: u64,
    pub pickup_location: String,
    pub delivery_location: String,
    pub status: DeliveryStatus,
    pub assigned_vehicle: Option<Pubkey>,
    pub created_at: i64,
    pub accepted_at: Option<i64>,
    pub completed_at: Option<i64>,
}
impl Delivery { pub const LEN: usize = 1 + 8 + 32 + 8 + (4 + 64) + (4 + 64) + 1 + (1 + 32) + 8 + (1 + 8) + (1 + 8); }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow occurred")]
    MathOverflow,
    #[msg("Config is inactive")]
    ConfigInactive,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Vehicle not available")]
    VehicleNotAvailable,
    #[msg("Invalid delivery status")]
    InvalidDeliveryStatus,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid treasury")]
    InvalidTreasury,
}
