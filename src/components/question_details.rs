use yew::prelude::*;

use super::questions_list::Question;

#[derive(Clone, Properties, PartialEq)]
pub struct QuestionsDetailsProps {
    pub question: Question,
}

#[function_component(QuestionDetails)]
pub fn question_details(QuestionsDetailsProps { question }: &QuestionsDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ question.question.clone() }</h3>
        </div>
    }
}