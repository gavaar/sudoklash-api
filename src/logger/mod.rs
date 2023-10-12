// todo: move this from here
use dotenv::dotenv;

pub fn init_logger() -> () {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "actix_web=info");
  }
  dotenv().ok();
  env_logger::init();
}
