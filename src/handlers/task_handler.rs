use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use crate::models::task::{Task, NewTask};
use crate::schema::tasks::dsl::*;
use crate::Pool;

#[get("/tasks")]
async fn get_tasks(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let task_list = web::block(move || tasks.load::<Task>(&conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(task_list))
}

#[post("/tasks")]
async fn add_task(pool: web::Data<Pool>, item: web::Json<NewTask>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let new_task = NewTask {
        title: item.title.clone(),
        description: item.description.clone(),
    };

    let inserted_task = web::block(move || {
        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(&conn)?;
        tasks.order(id.desc()).first(&conn)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(inserted_task))
}

#[put("/tasks/{id}")]
async fn update_task(pool: web::Data<Pool>, task_id: web::Path<i32>, item: web::Json<Task>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let updated_task = web::block(move || {
        diesel::update(tasks.find(task_id.into_inner()))
            .set((title.eq(&item.title), description.eq(&item.description), completed.eq(&item.completed)))
            .execute(&conn)?;
        tasks.find(task_id.into_inner()).first(&conn)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(updated_task))
}

#[delete("/tasks/{id}")]
async fn delete_task(pool: web::Data<Pool>, task_id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let _ = web::block(move || diesel::delete(tasks.find(task_id.into_inner())).execute(&conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().finish())
}
