extern crate utools;

use  utools::crontab;
use  utools::local_cache;

#[crontab(cron = "*/5 * * * * *", desc = "每5s执行一次")]
fn test1() {
    println!("test1");
}

#[local_cache(expire = 60, desc = "缓存更新周期60s")]
fn test2() {
    println!("test2");
}

fn main() {
    test1();
    test2();
}
