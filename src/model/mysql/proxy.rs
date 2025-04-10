use crate::controller::init::config::Mysql;
use anyhow::Result;
use futures::executor::block_on;
use lazy_static::lazy_static;
use rbatis::RBatis;
use rbdc::pool::Pool;
use rbdc_mysql::MysqlDriver;
use std::ops::Deref;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct MYSQlPOOL(Option<RBatis>);

lazy_static! {
    static ref GLOBAL_MYSQL_CONF: RwLock<Mysql> = RwLock::new(Mysql::default());
}

/// init mysql cli
pub async fn init_mysql(conf: Mysql) -> Result<()> {
    let mut mysql = GLOBAL_MYSQL_CONF.write().await;
    mysql.host = conf.host;
    mysql.max_open = conf.max_open;
    mysql.idle_open = conf.idle_open;
    mysql.max_life_time = conf.max_life_time;
    mysql.timeout = conf.timeout;
    Ok(())
}

/// default for MYSQlPOOL
impl Default for MYSQlPOOL {
    fn default() -> Self {
        block_on(async {
            let rb = RBatis::new();
            {
                let conf = GLOBAL_MYSQL_CONF.read().await;
                rb.init(MysqlDriver {}, &conf.host)
                    .expect("mysql init fail");
            }
            let pool = rb.pool.get().unwrap();
            setting_mysql(pool.deref()).await;
            MYSQlPOOL(Some(rb))
        })
    }
}

/// set mysql params
async fn setting_mysql(cli: &dyn Pool) {
    let conf = GLOBAL_MYSQL_CONF.read().await;
    cli.set_timeout(Some(Duration::from_secs(conf.timeout)))
        .await;
    cli.set_conn_max_lifetime(Some(Duration::from_secs(conf.max_life_time)))
        .await;
    cli.set_max_idle_conns(conf.idle_open).await;
    cli.set_max_open_conns(conf.max_open).await;
}

impl MYSQlPOOL {
    pub async fn get<'a>() -> &'a RBatis {
        lazy_static! {
            pub static ref POOL: MYSQlPOOL = MYSQlPOOL::default();
        }
        POOL.0.as_ref().unwrap()
    }
}
