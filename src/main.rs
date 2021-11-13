use reqwasm::http::Request;
use yew::prelude::*;

mod components;

use components::{
    questions_list::*,
    question_details::*
};

#[function_component(App)]
fn app() -> Html {

    let questions = use_state(|| vec![]);
    {
        let questions = questions.clone();
        use_effect_with_deps(move |_| {
            let questions = questions.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("fetching questions");
                let fetched_questions: Vec<Question> = Request::get("/api/tests")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                questions.set(fetched_questions);
                log::info!("fetched questions");
            });
            || ()
        }, ());
    }

    let selected_question = use_state(|| None);

    let on_question_select = {
        let selected_question = selected_question.clone();
        log::info!("on_question_select: {:?}", selected_question);

        Callback::from(move |question: Question| {
            log::info!("in callback {:?}", question);
            selected_question.set(Some(question))
        })
    };
    let details = selected_question.as_ref().map(|question| html! {
        <QuestionDetails question={question.clone()} />
    });

    html!(
        <>
            <h1>{ "Remember" }</h1>
            <div>
                <h3>{"Questions"}</h3>
                <QuestionsList questions={(*questions).clone()} on_click={on_question_select.clone()} />
            </div>
            {for details}
        </>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("entered main");
    yew::start_app::<App>();
}
