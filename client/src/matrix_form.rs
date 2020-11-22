use seed::{prelude::*, *};
use std::borrow::Cow;
use std::mem;
use web_sys::{
    self,
    console::{log_1, log_2},
    FormData,
};

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
    fn to_form_data(&self) -> Result<web_sys::FormData, JsValue> {
        let form_data = web_sys::FormData::new()?;
        form_data.append_with_str("title", &self.title)?;
        for i in 0..self.values.len() {
            form_data.append_with_str(i.to_string().as_str(), &format!("{}", self.values[i]));
        }
        Ok(form_data)
    }
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
            values: to_array(vec![0.; 9]),
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
    TitleChanged(String),
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
    //ServerResponded(fetch::Result<String>),
    Fetched(fetch::Result<shared::Quaternion>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(title) => model.form_mut().title = title,
        Msg::MatrixChanged0(value) => {
            model.form_mut().values[0] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged1(value) => {
            model.form_mut().values[1] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged2(value) => {
            model.form_mut().values[2] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged3(value) => {
            model.form_mut().values[3] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged4(value) => {
            model.form_mut().values[4] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged5(value) => {
            model.form_mut().values[5] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged6(value) => {
            model.form_mut().values[6] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged7(value) => {
            model.form_mut().values[7] = value.parse::<f64>().unwrap_or_default()
        }
        Msg::MatrixChanged8(value) => {
            model.form_mut().values[8] = value.parse::<f64>().unwrap_or_default()
        }
        //Msg::FormSubmitted(id) => {
        //    let form = mem::take(model.form_mut());
        //    let form_data = form.to_form_data().expect("create from data from form");
        //    orders.perform_cmd(async { Msg::ServerResponded(send_request(form_data).await) });
        //    *model = Model::WaitingForResponse(form);
        //    log!(format!("Form {} submitted.", id));
        //}
        Msg::FormSubmitted(id) => {
            let form = mem::take(model.form_mut());
            let rot_matrix = form.to_rotation_matrix().unwrap();
            orders.perform_cmd(async { Msg::Fetched(send_rot_matrix(rot_matrix).await) });
            *model = Model::WaitingForResponse(form);
            log!("Rotation Matrix emitted. Awaiting Quaternion.");
        }
        //Msg::ServerResponded(Ok(response_data)) => {
        //    *model = Model::ReadyToSubmit(Form::default());
        //    clear_file_input();
        //    log_2(
        //        &"%cResponse data:".into(),
        //        &"background: yellow; color: black".into(),
        //    );
        //    log_1(&response_data.into());
        //}
        //Msg::ServerResponded(Err(fetch_error)) => {
        //    *model = Model::ReadyToSubmit(mem::take(model.form_mut()));
        //    error!("Request failed!", fetch_error);
        //}
        Msg::Fetched(Ok(response_data)) => {
            *model = Model::ReadyToSubmit(Form::default());
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

async fn send_request(form: FormData) -> fetch::Result<String> {
    Request::new(get_request_url())
        .method(fetch::Method::Post)
        .body(form.into())
        .fetch()
        .await?
        .text()
        .await
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

#[allow(clippy::option_map_unit_fn)]
fn clear_file_input() {
    seed::document()
        .get_element_by_id("form-file")
        .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|file_input| {
            // Note: `file_input.set_files(None)` doesn't work
            file_input.set_value("")
        });
}

// ------ ------
//     View
// ------ ------

fn view_form_field(mut label: Node<Msg>, control: Node<Msg>) -> Node<Msg> {
    label.add_style("margin-right", unit!(7, px));
    div![
        style! {
          "margin-bottom" => unit!(7, px),
          "display" => "flex",
        },
        label,
        control
    ]
}

fn view_from_table(model: &Model) -> Node<Msg> {
    table!(
        tr!(),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged0),
                attrs! {
                    At::Id => "mat_value_0",
                    At::Value => model.form().values[0],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged1),
                attrs! {
                    At::Id => "mat_value_1",
                    At::Value => model.form().values[1],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged2),
                attrs! {
                    At::Id => "mat_value_2",
                    At::Value => model.form().values[2],
                    At::Required => true.as_at_value(),
                }
            ]),
        ),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged3),
                attrs! {
                    At::Id => "mat_value_3",
                    At::Value => model.form().values[3],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged4),
                attrs! {
                    At::Id => "mat_value_4",
                    At::Value => model.form().values[4],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged5),
                attrs! {
                    At::Id => "mat_value_5",
                    At::Value => model.form().values[5],
                    At::Required => true.as_at_value(),
                }
            ]),
        ),
        tr!(
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged6),
                attrs! {
                    At::Id => "mat_value_6",
                    At::Value => model.form().values[6],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged7),
                attrs! {
                    At::Id => "mat_value_7",
                    At::Value => model.form().values[7],
                    At::Required => true.as_at_value(),
                }
            ]),
            th!(input![
                input_ev(Ev::Input, Msg::MatrixChanged8),
                attrs! {
                    At::Id => "mat_value_8",
                    At::Value => model.form().values[8],
                    At::Required => true.as_at_value(),
                }
            ]),
        ),
    )
}

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    let btn_enabled = matches!(model, Model::ReadyToSubmit(form) if !form.title.is_empty() || form.values.iter().any(|value| value.is_infinite()|| value.is_nan()));

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
        view_form_field(
            label!["Title:", attrs! {At::For => "form-title" }],
            input![
                input_ev(Ev::Input, Msg::TitleChanged),
                attrs! {
                    At::Id => "form-title",
                    At::Value => model.form().title,
                    At::Required => true.as_at_value(),
                }
            ]
        ),
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
