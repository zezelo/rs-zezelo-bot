pub enum DiscordCommand {
    Evaluate,
}

impl DiscordCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiscordCommand::Evaluate => "evaluate",
        }
    }
}
pub enum DiscordCustomId {
    Evaluate,
    CreateEvaluateModal,
    EvaluateCommunication,
    EvaluateTeamWork,
    EvaluateBehaviour,
    EvaluateGrenade,
    EvaluateToxicity,
    EvaluateCommentary,
}

impl DiscordCustomId {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Evaluate => "evaluate",
            Self::CreateEvaluateModal => "create_evaluate_modal",
            Self::EvaluateBehaviour => "evaluate_behaviour",
            Self::EvaluateCommunication => "evaluate_communication",
            Self::EvaluateTeamWork => "evaluate_team_work",
            Self::EvaluateGrenade => "evaluate_grenade",
            Self::EvaluateToxicity => "evaluate_toxicity",
            Self::EvaluateCommentary => "evaluate_commentary",
        }
    }
}
