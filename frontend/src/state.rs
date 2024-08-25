use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_bootstrap::util::Color;

use crate::api::{get_info, submit_answers};
use crate::models::{AlertInfo, Info, Question, QuizResponse};

#[derive(Default, PartialEq, Clone)]
pub struct State {
    pub header: String,
    pub questions: Vec<Question>,
    pub answers: HashMap<String, String>,
    pub alert_info: Option<AlertInfo>,
}

pub enum Action {
    Info(Info),
    Answer(String, String),
    AlertInfo(Option<AlertInfo>),
    QuizResponse(QuizResponse),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Info(info) => Rc::new(State {
                header: info.title,
                questions: info.questions,
                ..(*self).clone()
            }),
            Action::Answer(id, answer) => {
                let mut new_state = (*self).clone();
                new_state.answers.insert(id.clone(), answer.clone());
                LocalStorage::set(id, &answer).unwrap_or_default();
                Rc::new(new_state)
            }
            Action::AlertInfo(info) => Rc::new(State {
                alert_info: info,
                ..(*self).clone()
            }),
            Action::QuizResponse(response) => {
                let color = if response.status {
                    Color::Success
                } else {
                    Color::Secondary
                };
                Rc::new(State {
                    alert_info: Some(AlertInfo {
                        color,
                        text: format!(
                            "本次测验总得分为 {}。<br/>{}",
                            response.score, response.message
                        ),
                    }),
                    ..(*self).clone()
                })
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct AppContext {
    pub state: UseReducerHandle<State>,
}

impl AppContext {
    pub fn new(state: UseReducerHandle<State>) -> Self {
        Self { state }
    }

    pub fn load_stored_answers(&self) {
        let state = self.state.clone();
        let stored_answers: HashMap<String, String> = LocalStorage::get_all().unwrap_or_default();
        for (key, value) in stored_answers {
            state.dispatch(Action::Answer(key, value));
        }
    }

    pub fn fetch_info(&self) {
        let state = self.state.clone();
        spawn_local(async move {
            match get_info().await {
                Ok(info) => state.dispatch(Action::Info(info)),
                Err(err) => state.dispatch(Action::AlertInfo(Some(AlertInfo {
                    color: Color::Danger,
                    text: err,
                }))),
            }
        });
    }

    pub fn submit_answers(&self) {
        let state = self.state.clone();
        let answers = state.answers.clone();
        spawn_local(async move {
            match submit_answers(&answers).await {
                Ok(response) => state.dispatch(Action::QuizResponse(response)),
                Err(err) => state.dispatch(Action::AlertInfo(Some(AlertInfo {
                    color: Color::Danger,
                    text: err,
                }))),
            }
        });
    }
}

#[hook]
pub fn use_app_context() -> AppContext {
    use_context::<AppContext>().expect("No context found!")
}
