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

fn get_request_url() -> impl Into<Cow<'static, str>> {
    "/api/matrix"
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Debug)]
pub struct Form {
    title: String,
}

impl Form {
    fn to_form_data(&self) -> Result<web_sys::FormData, JsValue> {
        let form_data = web_sys::FormData::new()?;
        form_data.append_with_str("title", &self.title)?;
        Ok(form_data)
    }
}

pub enum Model {
    ReadyToSubmit(Form),
    WaitingForResponse(Form),
}

impl Default for Model {
    fn default() -> Self {
        Self::ReadyToSubmit(Form {
            title: "I'm title".into(),
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
    ServerResponded(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(title) => model.form_mut().title = title,
        Msg::FormSubmitted(id) => {
            let form = mem::take(model.form_mut());
            let form_data = form.to_form_data().expect("create from data from form");
            orders.perform_cmd(async { Msg::ServerResponded(send_request(form_data).await) });
            *model = Model::WaitingForResponse(form);
            log!(format!("Form {} submitted.", id));
        }
        Msg::ServerResponded(Ok(response_data)) => {
            *model = Model::ReadyToSubmit(Form::default());
            clear_file_input();
            log_2(
                &"%cResponse data:".into(),
                &"background: yellow; color: black".into(),
            );
            log_1(&response_data.into());
        }
        Msg::ServerResponded(Err(fetch_error)) => {
            *model = Model::ReadyToSubmit(mem::take(model.form_mut()));
            error!("Request failed!", fetch_error);
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

pub fn view(model: &Model, intro: impl FnOnce(&str, &str) -> Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    let btn_enabled = matches!(model, Model::ReadyToSubmit(form) if !form.title.is_empty());

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
        table!(
            tr!(th!(input![
                input_ev(Ev::Input, Msg::TitleChanged)]), th!("TMP2")),
            tr!(th!("TMP"), th!("TMP2")),
            /*
                      <tr>
              <th>Firstname</th>
              <th>Lastname</th>
              <th>Age</th>
            </tr>
            <tr>
              <td>Jill</td>
              <td>Smith</td>
              <td>50</td>
            </tr>
            <tr>
              <td>Eve</td>
              <td>Jackson</td>
              <td>94</td>
            </tr>
            */
        ),
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
