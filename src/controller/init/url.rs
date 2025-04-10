use crate::controller::init::config::Url;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    static ref GLOBAL_URL: RwLock<Url> = RwLock::new(Url::default());
}

/// init global url
pub async fn init_url(conf: Url) {
    let mut url = GLOBAL_URL.write().await;
    url.cabinet = conf.cabinet
}
