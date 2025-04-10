use crate::controller::init::config::Clickhouse;
use anyhow::Result;
use clickhouse_rs::Pool;
use futures::executor::block_on;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    static ref GLOBAL_CK_CONF: RwLock<Clickhouse> = RwLock::new(Clickhouse::default());
}
pub async fn init_clickhouse<'a>(conf: Clickhouse) -> Result<()> {
    let mut ck = GLOBAL_CK_CONF.write().await;
    ck.host = conf.host;
    Ok(())
}

#[derive(Debug)]
pub struct CKPool(Option<Pool>);

impl Default for CKPool {
    fn default() -> Self {
        block_on(async {
            let conf = GLOBAL_CK_CONF.read().await;
            let cli = Pool::new(&*conf.host);
            CKPool(Option::from(cli))
        })
    }
}

/// get client query
///     let mut client = CKPool::get().await.get_handle().await.unwrap();
///     let block = client.query("SELECT * FROM payment").fetch_all().await?;
///
///     for row in block.rows() {
///         let id: u32             = row.get("customer_id")?;
///         let amount: u32         = row.get("amount")?;
///         let name: Option<&str>  = row.get("account_name")?;
///         println!("Found payment {}: {} {:?}", id, amount, name);
///     }
impl CKPool {
    pub async fn get<'a>() -> &'a Pool {
        lazy_static! {
            pub static ref POOL: CKPool = CKPool::default();
        }
        POOL.0.as_ref().unwrap()
    }
}
