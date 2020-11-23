use seed::{prelude::*, *};
use std::borrow::Cow;
use std::mem;

pub const TITLE: &str = "Matrix";
pub const DESCRIPTION: &str = "Rotation Matrix - fill form and be happy!";
const MAT_LEN: usize = 9;

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/matrix"
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Debug)]
pub struct Form {
    title: String,
    values: [f64; 9],
    response_data: Option<shared::Quaternion>,
}

impl Form {
    fn to_rotation_matrix(&self) -> Option<shared::RotationMatrix> {
        Some(shared::RotationMatrix {
            values: self.values.clone(),
        })
    }
}

pub enum Model {
    ReadyToSubmit(Form),
    WaitingForResponse(Form),
}

use std::convert::TryInto;
fn to_array<T>(v: Vec<T>) -> [T; MAT_LEN] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()))
}

impl Default for Model {
    fn default() -> Self {
        Self::ReadyToSubmit(Form {
            title: "Title".into(),
            values: to_array(vec![0.00; 9]),
            response_data: None,
        })
    }
}

impl Model {
    const fn form(&self) -> &Form {
        match self {
            Self::ReadyToSubmit(form) | Self::WaitingForResponse(form) => form,
        }
    }
    fn form_mut(&mut self) -> &mut Form {
        match self {
            Self::ReadyToSubmit(form) | Self::WaitingForResponse(form) => form,
        }
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    FormSubmitted(String),
    MatrixChanged0(String),
    MatrixChanged1(String),
    MatrixChanged2(String),
    MatrixChanged3(String),
    MatrixChanged4(String),
    MatrixChanged5(String),
    MatrixChanged6(String),
    MatrixChanged7(String),
    MatrixChanged8(String),
    Fetched(fetch::Result<shared::Quaternion>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::MatrixChanged0(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[0] = parsed_val;
            }
        }
        Msg::MatrixChanged1(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[1] = parsed_val;
            }
        }
        Msg::MatrixChanged2(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[2] = parsed_val;
            }
        }
        Msg::MatrixChanged3(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[3] = parsed_val;
            }
        }
        Msg::MatrixChanged4(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[4] = parsed_val;
            }
        }
        Msg::MatrixChanged5(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[5] = parsed_val;
            }
        }
        Msg::MatrixChanged6(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[6] = parsed_val;
            }
        }
        Msg::MatrixChanged7(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[7] = parsed_val;
            }
        }
        Msg::MatrixChanged8(value) => {
            if let Ok(parsed_val) = value.parse::<f64>() {
                model.form_mut().values[8] = parsed_val;
            }
        }
        Msg::FormSubmitted(id) => {
            let form = mem::take(model.form_mut());
            let rot_matrix = form.to_rotation_matrix().unwrap();
            orders.perform_cmd(async { Msg::Fetched(send_rot_matrix(rot_matrix).await) });
            *model = Model::WaitingForResponse(form);
            log!("Rotation Matrix emitted. Awaiting Quaternion.");
        }
        Msg::Fetched(Ok(response_data)) => {
            *model = Model::ReadyToSubmit(mem::take(model.form_mut()));
            let q = response_data.clone();
            model.form_mut().response_data = Some(response_data);
            log!("Got Quaternion: x:{}, y:{}, z:{}, w:{}", q.x, q.y, q.z, q.w);
        }
        Msg::Fetched(Err(fetch_error)) => {
            *model = Model::ReadyToSubmit(mem::take(model.form_mut()));
            model.form_mut().response_data = None;
            log!("Example_A error:", fetch_error);
            orders.skip();
        }
    }
}

async fn send_rot_matrix(mat: shared::RotationMatrix) -> fetch::Result<shared::Quaternion> {
    Request::new(get_request_url())
        .method(Method::Post)
        .json(&mat)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     View
// ------ ------

fn view_from_table(model: &Model) -> Node<Msg> {
    table!(
        tr!(),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged0),
                attrs! {
                    At::Id => "mat_value_0",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged1),
                attrs! {
                    At::Id => "mat_value_1",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged2),
                attrs! {
                    At::Id => "mat_value_2",
                }
            ]),
        ),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged3),
                attrs! {
                    At::Id => "mat_value_3",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged4),
                attrs! {
                    At::Id => "mat_value_4",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged5),
                attrs! {
                    At::Id => "mat_value_5",
                }
            ]),
        ),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged6),
                attrs! {
                    At::Id => "mat_value_6",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged7),
                attrs! {
                    At::Id => "mat_value_7",
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged8),
                attrs! {
                    At::Id => "mat_value_8",
                }
            ]),
        ),
    )
}

fn view_quaternion(quat: &Option<shared::Quaternion>) -> Node<Msg> {
    let quat = match quat {
        Some(quat) => quat,
        None => return empty![],
    };
    div![format!(
        "quat: x: {}, y: {}, z: {}, w: {}",
        quat.x, quat.y, quat.z, quat.w
    )]
}

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    let btn_enabled = matches!(model, Model::ReadyToSubmit(form) if form.values.iter().any(|value| !value.is_nan()));
    let form_id = "A_FORM".to_string();
    let form = form![
        style! {
            St::Display => "flex",
            St::FlexDirection => "column",
        },
        ev(Ev::Submit, move |event| {
            event.prevent_default();
            Msg::FormSubmitted(form_id)
        }),
        view_from_table(model),
        view_quaternion(&model.form().response_data),
        button![
            style! {
                "padding" => format!{"{} {}", px(2), px(12)},
                "background-color" => if btn_enabled { CSSValue::from("aquamarine") } else { CSSValue::Ignored },
            },
            attrs! {At::Disabled => not(btn_enabled).as_at_value()},
            "Submit"
        ]
    ];

    nodes![intro(TITLE, DESCRIPTION), form]
}
