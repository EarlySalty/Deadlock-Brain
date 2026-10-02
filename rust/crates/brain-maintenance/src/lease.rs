use anyhow::{ensure, Result};
use brain_contracts::maintenance::{
    MaintenanceLease, MaintenanceStorePort, MAX_MAINTENANCE_LEASE_MS,
};
use std::{future::Future, time::Duration};

/// Ergebnisübergabe erst nach letzter erfolgreicher Erneuerung des Job-Fences.
/// Kanonische Publikation benötigt zusätzlich den gemeinsamen PG-Guard.
pub async fn while_leased<T, F>(
    store: &dyn MaintenanceStorePort,
    lease: &mut MaintenanceLease,
    ttl_ms: u64,
    work: F,
) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    ensure!(
        (3000..=MAX_MAINTENANCE_LEASE_MS).contains(&ttl_ms),
        "Ungültige Wartungslease"
    );
    *lease = store.renew_maintenance(lease, ttl_ms).await?;
    let mut interval = tokio::time::interval(Duration::from_millis(ttl_ms / 3));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    tokio::pin!(work);
    loop {
        tokio::select! {
            result=&mut work => { *lease=store.renew_maintenance(lease,ttl_ms).await?; return result; },
            _=interval.tick() => { *lease=store.renew_maintenance(lease,ttl_ms).await?; }
        }
    }
}
