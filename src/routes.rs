use actix_web::web;
use crate::controllers::{task_controller, auth_controller, notification_controller};
use crate::middleware::JwtMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .wrap(JwtMiddleware)
            .route("", web::post().to(task_controller::create_task))
            .route("", web::get().to(task_controller::get_all_tasks))
            .route("/{id}", web::get().to(task_controller::get_task_by_id))
            .route("/{id}", web::put().to(task_controller::update_task))
            .route("/{id}", web::delete().to(task_controller::delete_task)),
    );
}
