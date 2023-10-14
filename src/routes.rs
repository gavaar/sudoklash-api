mod ping;
mod sessions;
mod users;

use actix_web::web;

pub fn init(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/v1")
    .service(ping::ping)
    .service(users::me)
    .configure(sessions::routes);

  conf.service(scope);
}
