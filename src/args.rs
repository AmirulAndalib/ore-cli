use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct BalanceArgs {
    #[arg(
        value_name = "ADDRESS",
        help = "The account address to fetch the balance of."
    )]
    pub address: Option<String>,

    #[command(subcommand)]
    pub command: Option<BalanceCommand>,

    #[arg(
        long,
        short,
        value_name = "POOL_URL",
        help = "The optional pool url to fetch the balance from."
    )]
    pub pool_url: Option<String>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum BalanceCommand {
    #[command(about = "Commit a pending pool balance to the chain.")]
    Commit(BalanceCommitArgs),
}

#[derive(Parser, Clone, Debug)]
pub struct BalanceCommitArgs {}

#[derive(Parser, Debug)]
pub struct BenchmarkArgs {
    #[arg(
        long,
        short,
        value_name = "THREAD_COUNT",
        help = "The number of cores to use during the benchmark",
        default_value = "1"
    )]
    pub cores: u64,
}

#[derive(Parser, Debug)]
pub struct BussesArgs {}

#[derive(Parser, Debug)]
pub struct ClaimArgs {
    #[arg(
        value_name = "AMOUNT",
        help = "The amount of rewards to claim. Defaults to max."
    )]
    pub amount: Option<f64>,

    #[arg(
        long,
        value_name = "WALLET_ADDRESS",
        help = "Wallet address to receive claimed tokens."
    )]
    pub to: Option<String>,

    #[arg(
        long,
        short,
        value_name = "POOL_URL",
        help = "The optional pool url to claim rewards from."
    )]
    pub pool_url: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CloseArgs {}

#[derive(Parser, Debug)]
pub struct ConfigArgs {}

#[cfg(feature = "admin")]
#[derive(Parser, Debug)]
pub struct InitializeArgs {}

#[derive(Parser, Debug)]
pub struct MineArgs {
    #[arg(
        long,
        short,
        value_name = "CORES_COUNT",
        help = "The number of CPU cores to allocate to mining.",
        default_value = "1"
    )]
    pub cores: u64,

    #[arg(
        long,
        short,
        value_name = "SECONDS",
        help = "The number seconds before the deadline to stop mining and start submitting.",
        default_value = "5"
    )]
    pub buffer_time: u64,

    #[arg(
        long,
        value_name = "MINT_ADDRESS",
        help = "The token to apply as boost #1"
    )]
    pub boost_1: Option<String>,

    #[arg(
        long,
        value_name = "MINT_ADDRESS",
        help = "The token to apply as boost #2"
    )]
    pub boost_2: Option<String>,

    #[arg(
        long,
        value_name = "MINT_ADDRESS",
        help = "The token to apply as boost #3"
    )]
    pub boost_3: Option<String>,

    #[arg(
        long,
        short,
        value_name = "POOL_URL",
        help = "The optional pool url to join and forward solutions to."
    )]
    pub pool_url: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ProofArgs {
    #[arg(value_name = "ADDRESS", help = "The address of the proof to fetch.")]
    pub address: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RewardsArgs {}

#[derive(Parser, Debug)]
pub struct StakeArgs {
    #[command(subcommand)]
    pub command: Option<StakeCommand>,

    #[arg(value_name = "MINT_ADDRESS", help = "The mint to stake with.")]
    pub mint: String,

    #[arg(
        long,
        short,
        value_name = "POOL_URL",
        help = "The pool url to stake with."
    )]
    pub pool_url: Option<String>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum StakeCommand {
    #[command(about = "Deposit tokens into a stake account.")]
    Deposit(StakeDepositArgs),

    #[command(about = "Withdraw tokens from a stake account.")]
    Withdraw(StakeWithdrawArgs),
}

#[derive(Parser, Clone, Debug)]
pub struct StakeDepositArgs {
    #[arg(
        value_name = "AMOUNT",
        help = "The amount of stake to deposit. Defaults to max."
    )]
    pub amount: Option<f64>,

    #[arg(
        long,
        value_name = "TOKEN_ACCOUNT_ADDRESS",
        help = "Token account to deposit from. Defaults to the associated token account."
    )]
    pub token_account: Option<String>,
}

#[derive(Parser, Clone, Debug)]
pub struct StakeWithdrawArgs {
    #[arg(
        value_name = "AMOUNT",
        help = "The amount of stake to withdraw. Defaults to max."
    )]
    pub amount: Option<f64>,

    #[arg(
        long,
        value_name = "TOKEN_ACCOUNT_ADDRESS",
        help = "Token account to withdraw to. Defaults to the associated token account."
    )]
    pub token_account: Option<String>,
}

#[derive(Parser, Debug)]
pub struct TransferArgs {
    #[arg(value_name = "AMOUNT", help = "The amount of ORE to transfer.")]
    pub amount: f64,

    #[arg(
        value_name = "RECIPIENT_ADDRESS",
        help = "The account address of the receipient."
    )]
    pub to: String,
}

#[derive(Parser, Debug)]
pub struct UnstakeArgs {
    #[arg(
        value_name = "AMOUNT",
        help = "The amount of the token to unstake. Defaults to max."
    )]
    pub amount: Option<f64>,

    #[arg(value_name = "MINT_ADDRESS", help = "The mint to unstake.")]
    pub mint: String,

    #[arg(
        long,
        value_name = "TOKEN_ACCOUNT_ADDRESS",
        help = "Token account to receive unstaked funds. Defaults to the associated token account."
    )]
    pub token_account: Option<String>,

    #[arg(
        long,
        short,
        value_name = "POOL_URL",
        help = "The optional pool url to unstake from."
    )]
    pub pool_url: Option<String>,
}

#[derive(Parser, Debug)]
pub struct UpgradeArgs {
    #[arg(
        value_name = "AMOUNT",
        help = "The amount of ORE to upgrade from v1 to v2. Defaults to max."
    )]
    pub amount: Option<f64>,
}
