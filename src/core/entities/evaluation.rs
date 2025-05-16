#[derive(Default)]
pub struct CreateUserEvaluation {
    pub evaluator_id: String,
    pub player_id: String,
    pub communication: String,
    pub teamplay: String,
    pub utility_usage: String,
    pub behavior: String,
    pub comment: Option<String>,
}
