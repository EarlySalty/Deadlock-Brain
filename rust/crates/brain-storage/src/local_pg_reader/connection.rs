//! Drive the already-used tokio-postgres transport in its owning pooled connection.
//! No spawned driver, helper thread, extra pool, or per-operation runtime is involved.
//! Unlike postgres::Config::connect, the complete startup and each wire operation are
//! bounded here. A timed-out transport is closed before pool capacity is returned.
use super::{error, request_check, unavailable, ConnectionConfig};
use brain_contracts::{PortError, RequestDeadline};
use std::{
    future::Future,
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;
use tokio_postgres::{types::ToSql, Config, Connection, NoTls, Row, Socket};

type Wire = Connection<Socket, tokio_postgres::tls::NoTlsStream>;

struct Driver {
    connection: Option<Wire>,
    // One current-thread runtime per physical pooled connection, as in postgres::Client.
    // It is reused for every operation; no tasks or blocking workers are ever spawned.
    runtime: Option<Runtime>,
    deadline: Option<RequestDeadline>,
    startup_end: Option<Instant>,
    statement_timeout: Duration,
}
impl Drop for Driver {
    fn drop(&mut self) {
        self.connection.take(); // Close the Unix socket before dropping the I/O driver.
        if let Some(runtime) = self.runtime.take() {
            // No spawned work exists to detach. This also permits pool destruction from
            // an async composition root without entering/blocking a nested runtime.
            runtime.shutdown_background();
        }
    }
}
async fn expired(end: Instant, deadline: Option<&RequestDeadline>) -> PortError {
    loop {
        if deadline.is_some_and(|deadline| deadline.check().is_err()) {
            return PortError::BudgetExceeded;
        }
        let remaining = end.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return unavailable("local PostgreSQL operation timed out");
        }
        let wait = if deadline.is_some() {
            remaining.min(Duration::from_millis(10))
        } else {
            remaining
        };
        tokio::time::sleep(wait).await;
    }
}
impl Driver {
    fn run<T>(
        &mut self,
        operation: impl Future<Output = Result<T, tokio_postgres::Error>>,
    ) -> Result<T, PortError> {
        request_check(self.deadline.as_ref())?;
        let end = self
            .startup_end
            .unwrap_or_else(|| Instant::now() + self.statement_timeout);
        let wire = self
            .connection
            .as_mut()
            .ok_or_else(|| unavailable("local PostgreSQL connection closed"))?;
        let result = self
            .runtime
            .as_ref()
            .expect("owned connection runtime")
            .block_on(async {
                tokio::select! {
                    biased;
                    reason = expired(end, self.deadline.as_ref()) => Err(reason),
                    value = operation => value.map_err(error),
                    _ = wire => Err(unavailable("local PostgreSQL connection closed")),
                }
            });
        // Never reuse a partially executed/timed-out protocol stream. Closing also
        // rolls back an open transaction; no cleanup SQL starts after cancellation.
        if result.is_err() {
            self.connection.take();
        }
        result
    }
}

pub(super) struct Client {
    client: tokio_postgres::Client,
    driver: Driver,
}
impl Client {
    pub(super) fn connect(
        config: &ConnectionConfig,
        deadline: Option<&RequestDeadline>,
    ) -> Result<Self, PortError> {
        request_check(deadline)?;
        let end = Instant::now() + config.connect_timeout;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| unavailable("local PostgreSQL I/O driver unavailable"))?;
        let mut driver = Driver {
            connection: None,
            runtime: Some(runtime),
            deadline: deadline.cloned(),
            startup_end: Some(end),
            statement_timeout: config.statement_timeout,
        };
        let mut options = Config::new();
        options
            .host_path(&config.socket)
            .port(config.port)
            .user(&config.user)
            .dbname(&config.database)
            .application_name("deadlock-brain-reader")
            .ssl_mode(tokio_postgres::config::SslMode::Disable)
            .connect_timeout(config.connect_timeout);
        if let Some(password) = &config.password {
            options.password(password);
        }
        let (client, connection) = driver
            .runtime
            .as_ref()
            .expect("owned connection runtime")
            .block_on(async {
                tokio::select! {
                    biased;
                    reason = expired(end, deadline) => Err(reason),
                    result = options.connect(NoTls) => result.map_err(error),
                }
            })?;
        driver.connection = Some(connection);
        let mut client = Self { client, driver };
        // Fixed server-side safeguards are installed once. The local transport guard
        // pays every subsequent operation from the original request budget, avoiding
        // extra SET round trips on every checkout and every query.
        client.batch_execute(&format!(
            "SET statement_timeout='{}ms'; SET lock_timeout='{}ms'",
            config.statement_timeout.as_millis(),
            config.lock_timeout.as_millis()
        ))?;
        client.driver.startup_end = None;
        Ok(client)
    }
    pub(super) fn set_deadline(
        &mut self,
        deadline: Option<&RequestDeadline>,
    ) -> Result<(), PortError> {
        request_check(deadline)?;
        self.driver.deadline = deadline.cloned();
        Ok(())
    }
    pub(super) fn is_closed(&self) -> bool {
        self.driver.connection.is_none() || self.client.is_closed()
    }
    pub(super) fn batch_execute(&mut self, sql: &str) -> Result<(), PortError> {
        self.driver.run(self.client.batch_execute(sql))
    }
    pub(super) fn query(
        &mut self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, PortError> {
        self.driver.run(self.client.query(sql, params))
    }
    pub(super) fn query_one(
        &mut self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, PortError> {
        self.driver.run(self.client.query_one(sql, params))
    }
    pub(super) fn query_opt(
        &mut self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, PortError> {
        self.driver.run(self.client.query_opt(sql, params))
    }
    pub(super) fn execute(
        &mut self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, PortError> {
        self.driver.run(self.client.execute(sql, params))
    }
    pub(super) fn transaction(
        &mut self,
        read_only: bool,
        repeatable_read: bool,
    ) -> Result<Transaction<'_>, PortError> {
        let sql = match (read_only, repeatable_read) {
            (false, false) => "BEGIN",
            (true, false) => "BEGIN READ ONLY",
            (true, true) => "BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY",
            (false, true) => "BEGIN ISOLATION LEVEL REPEATABLE READ",
        };
        self.batch_execute(sql)?;
        Ok(Transaction {
            client: self,
            committed: false,
        })
    }
}

pub(super) struct Transaction<'a> {
    client: &'a mut Client,
    committed: bool,
}
impl Deref for Transaction<'_> {
    type Target = Client;
    fn deref(&self) -> &Client {
        self.client
    }
}
impl DerefMut for Transaction<'_> {
    fn deref_mut(&mut self) -> &mut Client {
        self.client
    }
}
impl Transaction<'_> {
    pub(super) fn commit(mut self) -> Result<(), PortError> {
        self.client.batch_execute("COMMIT")?;
        self.committed = true;
        Ok(())
    }
}
impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if !self.committed {
            // Failure is fail-closed, with no potentially blocking rollback in Drop.
            self.client.driver.connection.take();
        }
    }
}
