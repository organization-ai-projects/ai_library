use super::{router::route_to_expert, expert::MoeExpert};

/// Fonction pour Mixture of Experts avec toutes les IA.
pub fn moe_with_all_ai(experts: &[MoeExpert], input: &[f32]) -> f32 {
    let idx = route_to_expert(input);
    experts[idx].decide(input)
}
