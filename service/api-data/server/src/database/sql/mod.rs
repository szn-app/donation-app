// raw Postgres SQL parametrized queries (which tokio-postgres creates PREPARE statements from and execution)

pub const GET_ACCOUNTS: &str = include_str!("./GET_ACCOUNTS.sql");
pub const ADD_ACCOUNT: &str = include_str!("./ADD_ACCOUNT.sql");

pub const GET_TESTS: &str = include_str!("./GET_TESTS.sql");
