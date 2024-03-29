use jelly::actix_web::web::{resource, scope, ServiceConfig};

mod views;
mod models;

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        scope("/qa/")
            // Index
            .service(resource("{id}").to(views::question_detail))
            .service(resource("").to(views::index)),
    );
}