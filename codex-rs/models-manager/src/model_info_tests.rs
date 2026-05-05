use super::*;
use crate::ModelsManagerConfig;
use codex_protocol::config_types::Personality;
use pretty_assertions::assert_eq;

fn personality_enabled_config() -> ModelsManagerConfig {
    ModelsManagerConfig {
        personality_enabled: true,
        ..Default::default()
    }
}

#[test]
fn reasoning_summaries_override_true_enables_support() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(true),
        ..personality_enabled_config()
    };

    let updated = with_config_overrides(model.clone(), &config);
    let mut expected = model;
    expected.supports_reasoning_summaries = true;

    assert_eq!(updated, expected);
}

#[test]
fn reasoning_summaries_override_false_does_not_disable_support() {
    let mut model = model_info_from_slug("unknown-model");
    model.supports_reasoning_summaries = true;
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(false),
        ..personality_enabled_config()
    };

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn reasoning_summaries_override_false_is_noop_when_model_is_false() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(false),
        ..personality_enabled_config()
    };

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn model_context_window_override_clamps_to_max_context_window() {
    let mut model = model_info_from_slug("unknown-model");
    model.context_window = Some(273_000);
    model.max_context_window = Some(400_000);
    let config = ModelsManagerConfig {
        model_context_window: Some(500_000),
        ..personality_enabled_config()
    };

    let updated = with_config_overrides(model.clone(), &config);
    let mut expected = model;
    expected.context_window = Some(400_000);

    assert_eq!(updated, expected);
}

#[test]
fn model_context_window_uses_model_value_without_override() {
    let mut model = model_info_from_slug("unknown-model");
    model.context_window = Some(273_000);
    model.max_context_window = Some(400_000);
    let config = personality_enabled_config();

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn fallback_model_instructions_use_standalone_personality_prompt() {
    let model = model_info_from_slug("unknown-model");
    let instructions = model.get_model_instructions(Some(Personality::Pragmatic));

    assert!(instructions.contains("You are a Goblin."));
    assert!(instructions.contains("# Personality"));
    assert!(instructions.contains("You happen to live in a terminal and work with code"));
    assert!(instructions.contains("You run inside the Goblins CLI"));
    assert!(instructions.contains("# Project Docs Spec"));
    assert!(instructions.contains("GOBLINS.md"));
    assert!(instructions.contains("Ship the thing"));
    assert!(instructions.contains("Failure is information."));
    assert!(instructions.contains("You are a Goblin, and that means something."));
    assert_eq!(instructions.matches("You are a Goblin.").count(), 1);
    assert!(!instructions.contains("You are a Goblins."));
    assert!(!instructions.contains("You are a Goblin:"));
}

#[test]
fn personality_none_uses_goblin_base_prompt() {
    let model = model_info_from_slug("unknown-model");
    let instructions = model.get_model_instructions(Some(Personality::None));

    assert!(instructions.contains("You are a Goblin."));
    assert!(!instructions.contains("# Personality"));
    assert!(instructions.contains("# Project Docs Spec"));
}
