use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::task::Task;
use crate::schema::tasks::dsl::*;
use crate::DbPool;

pub async fn create_task(pool: web::Data<DbPool>, new_task: web::Json<Task>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let new_task = new_task.into_inner();

    diesel::insert_into(tasks)
        .values(&new_task)
        .execute(&conn)
        .expect("Error saving new task");

    HttpResponse::Created().json(new_task)
}

pub async fn get_all_tasks(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let all_tasks = tasks.load::<Task>(&conn).expect("Error loading tasks");

    HttpResponse::Ok().json(all_tasks)
}

pub async fn get_task_by_id(pool: web::Data<DbPool>, task_id: web::Path<Uuid>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let task = tasks.find(task_id.into_inner()).first::<Task>(&conn).ok();

    match task {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_task(pool: web::Data<DbPool>, task_id: web::Path<Uuid>, updated_task: web::Json<Task>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let updated_task = diesel::update(tasks.find(task_id.into_inner()))
        .set(&updated_task.into_inner())
        .get_result::<Task>(&conn)
        .expect("Error updating task");

    HttpResponse::Ok().json(updated_task)
}

pub async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<Uuid>) -> HttpResponse {
    let conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(tasks.find(task_id.into_inner()))
        .execute(&conn)
        .expect("Error deleting task");

    HttpResponse::NoContent().finish()
}
