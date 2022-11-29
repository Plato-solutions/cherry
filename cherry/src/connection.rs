use std::any::TypeId;
use std::collections::BTreeMap;
use std::time::Duration;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use sqlx::pool::PoolOptions;
use crate::ConnectOptions;
use crate::sqlx::Database;

use crate::types::{Pool, Result, };

static POOLS: OnceCell<BTreeMap<TypeId, Pool>> = OnceCell::new();

pub async fn setup_pools<T>(config: T) -> Result<()>
    where T: IntoIterator<Item = (TypeId, PoolConfig)>
{
    if POOLS.get().is_none()
    {
        let mut pools = BTreeMap::new();
        for (key, v) in config {
            pools.insert(key, v.to_pool().await?);
        }

        POOLS.set(pools).map_err(|_| anyhow!("Failed to set pools."))?;
    } else {
        if cfg!(any(test, feature = "test")) {
            return Ok(());
        } else {
            Err(anyhow!("Failed to set pools."))?;
        }
    }

    Ok(())
}

pub async fn setup_pools_with<T>(config: T) -> Result<()>
    where T: IntoIterator<Item = (TypeId, PoolConfig,crate::ConnectOptions)>
{
    if POOLS.get().is_none()
    {
        let mut pools = BTreeMap::new();
        for (key, v,o) in config {
            pools.insert(key, v.to_pool_with(o).await?);
        }

        POOLS.set(pools).map_err(|_| anyhow!("Failed to set pools."))?;
    } else {
        if cfg!(any(test, feature = "test")) {
            return Ok(());
        } else {
            Err(anyhow!("Failed to set pools."))?;
        }
    }

    Ok(())
}

pub fn get(type_id: TypeId) -> Result<&'static Pool> {
    let value = POOLS.get()
        .ok_or_else(|| anyhow!("Pools is empty."))?
        .get(&type_id)
        .ok_or_else(|| anyhow!("No pool found for key: {:?}", type_id))?;
    Ok(value)
}


/// Because pools is OnceCell, this renders that database nearly useless
pub async fn close(type_id: TypeId) -> Result<bool> {
    let value = POOLS.get()
        .ok_or_else(|| anyhow!("Pools is empty."))?
        .get(&type_id)
        .ok_or_else(|| anyhow!("No pool found for key: {:?}", type_id))?;

    value.close().await;

    Ok(value.is_closed())
}


#[cfg_attr(feature = "json", derive(serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PoolConfig {
    pub url: String,
    pub test_before_acquire: Option<bool>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub log_statements_level: Option<String>,
    pub log_slow_queries_level: Option<String>,
    pub log_slow_queries_duration: Option<Duration>,
    // after_connect: None,
    // before_acquire: None,
    // after_release: None,
    // fair: Option<bool>,
}

impl PoolConfig {

    pub(crate) async fn to_pool_with(&self,options: ConnectOptions) -> Result<Pool> {
        let pool_opts = self.pool_options();
        Ok( pool_opts.connect_with(options).await?)
    }

    pub(crate) async fn to_pool(&self) -> Result<Pool> {
        let pool_opts = self.pool_options();
        Ok(pool_opts.connect(self.url.as_str() ).await?)
    }

    fn pool_options<D:Database>(&self) -> PoolOptions<D> {
        let mut pool_opts = PoolOptions::new();
        if let Some(v) = self.test_before_acquire {
            pool_opts = pool_opts.test_before_acquire(v);
        }
        if let Some(v) = self.max_connections {
            pool_opts = pool_opts.max_connections(v);
        }
        if let Some(v) = self.min_connections {
            pool_opts = pool_opts.min_connections(v);
        }
        if let Some(v) = self.connect_timeout {
            pool_opts = pool_opts.connect_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.max_lifetime {
            pool_opts = pool_opts.max_lifetime(Duration::from_secs(v));
        }
        if let Some(v) = self.idle_timeout {
            pool_opts = pool_opts.idle_timeout(Duration::from_secs(v));
        }

        pool_opts
    }
}
