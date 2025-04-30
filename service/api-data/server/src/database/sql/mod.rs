// raw Postgres SQL parametrized queries (which tokio-postgres creates PREPARE statements from and execution)

pub const SQL_GET_ACCOUNTS: &str = include_str!("./get_accounts.sql");
pub const SQL_ADD_ACCOUNT: &str = include_str!("./add_account.sql");
