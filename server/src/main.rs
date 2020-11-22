use actix::prelude::*;
use actix_files::{Files, NamedFile};
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpServer, Result};
use futures::stream::StreamExt;
use nalgebra::{Matrix3, Quaternion, Rotation3, UnitQuaternion};
use std::fmt::Write;
use std::time;

mod count_actor;
use count_actor::{CountActor, MsgIncrement};

// ---- Apis ("/api/*") ----

#[post("send-message")]
async fn send_message(
    state: web::Data<State>,
    request_data: web::Json<shared::SendMessageRequestBody>,
) -> Result<web::Json<shared::SendMessageResponseBody>> {
    Ok(web::Json(shared::SendMessageResponseBody {
        ordinal_number: state
            .count_actor
            .send(MsgIncrement)
            .await
            .expect("send MsgIncrement"),
        text: request_data.text.clone(),
    }))
}

#[get("delayed-response/{delay}")]
async fn delayed_response(delay: web::Path<u64>) -> String {
    futures_timer::Delay::new(time::Duration::from_millis(*delay)).await;
    format!("Delay was set to {}ms.", delay)
}

#[post("form")]
async fn form(mut form: Multipart) -> String {
    let mut name_text_pairs: Vec<(String, String)> = Vec::new();
    while let Some(Ok(mut field)) = form.next().await {
        let field_name = field
            .content_disposition()
            .and_then(|cd| cd.get_name().map(ToString::to_string))
            .expect("Can't get field name!");

        let mut field_bytes: Vec<u8> = Vec::new();
        while let Some(Ok(bytes)) = field.next().await {
            for byte in bytes {
                field_bytes.push(byte)
            }
        }

        let field_text = String::from_utf8_lossy(&field_bytes).into_owned();
        name_text_pairs.push((field_name, field_text));
    }

    let mut output = String::new();
    for (name, text) in name_text_pairs {
        writeln!(&mut output, "{}: {}", name, text).unwrap();
        writeln!(&mut output, "___________________").unwrap();
    }
    output
}

//#[post("matrix")]
//async fn matrix_form(mut matrix: Multipart) -> String {
//    let mut name_text_pairs: Vec<(String, String)> = Vec::new();
//    while let Some(Ok(mut field)) = matrix.next().await {
//        let field_name = field
//            .content_disposition()
//            .and_then(|cd| cd.get_name().map(ToString::to_string))
//            .expect("Can't get field name!");
//
//        let mut field_bytes: Vec<u8> = Vec::new();
//        while let Some(Ok(bytes)) = field.next().await {
//            for byte in bytes {
//                field_bytes.push(byte)
//            }
//        }
//
//        let field_text = String::from_utf8_lossy(&field_bytes).into_owned();
//        name_text_pairs.push((field_name, field_text));
//    }
//
//    let mut output = String::new();
//    for (name, text) in name_text_pairs {
//        writeln!(&mut output, "{}: {}", name, text).unwrap();
//        writeln!(&mut output, "___________________").unwrap();
//    }
//    output
//}

#[post("matrix")]
async fn matrix(
    request_data: web::Json<shared::RotationMatrix>,
) -> Result<web::Json<shared::Quaternion>> {
    let flat_mat = &request_data.values;
    let m = Matrix3::new(
        flat_mat[0],
        flat_mat[1],
        flat_mat[2],
        flat_mat[3],
        flat_mat[4],
        flat_mat[5],
        flat_mat[6],
        flat_mat[7],
        flat_mat[8],
    );
    let rot_matrix = Rotation3::from_matrix(&m);
    let q = UnitQuaternion::from_rotation_matrix(&rot_matrix);
    Ok(web::Json(shared::Quaternion {
        x: q[0],
        y: q[1],
        z: q[2],
        w: q[3],
    }))
}

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/index.html")?)
}

struct State {
    count_actor: Addr<CountActor>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let count_actor_addr = CountActor(0).start();
    HttpServer::new(move || {
        App::new()
            .data(State {
                count_actor: count_actor_addr.clone(),
            })
            .service(
                web::scope("/api/")
                    .service(send_message)
                    .service(delayed_response)
                    .service(form)
                    .service(matrix)
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
