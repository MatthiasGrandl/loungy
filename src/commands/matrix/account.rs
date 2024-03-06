

use gpui::*;
use log::error;

use crate::{
    components::form::{Form, Input, InputKind},
    state::{StateViewBuilder, StateViewContext},
};

use super::client::Session;

#[derive(Clone)]
pub struct AccountCreationBuilder;

impl StateViewBuilder for AccountCreationBuilder {
    fn build(&self, context: &mut StateViewContext, cx: &mut WindowContext) -> AnyView {
        context.query.set_placeholder("Login...", cx);
        Form::new(
            vec![
                Input::new(
                    "username",
                    "Username",
                    InputKind::TextField {
                        placeholder: "@username:matrix.org".to_string(),
                        value: "".to_string(),
                        validate: Some(|v| v.is_empty().then_some("Username is required")),
                        password: false,
                    },
                    cx,
                ),
                Input::new(
                    "password",
                    "Password",
                    InputKind::TextField {
                        placeholder: "Enter password...".to_string(),
                        value: "".to_string(),
                        validate: Some(|v| v.is_empty().then_some("Password is required")),
                        password: true,
                    },
                    cx,
                ),
            ],
            move |values, actions, cx| {
                let username = values["username"].value::<String>();
                let password = values["password"].value::<String>();
                let actions = actions.clone();
                cx.spawn(move |mut cx| async move {
                    let mut actions_clone = actions.clone();
                    if let Err(err) = Session::login(username, password, actions, &mut cx).await {
                        error!("Failed to login: {}", err);
                        actions_clone
                            .toast
                            .error(&format!("Failed to login: {}", err), &mut cx);
                    }
                })
                .detach();

                //
            },
            context,
            cx,
        )
        .into()
    }
}
