use super::*;
use crate::ModelsManagerConfig;
use codex_protocol::config_types::Personality;
use pretty_assertions::assert_eq;

#[test]
fn reasoning_summaries_override_true_enables_support() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(true),
        ..Default::default()
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
        ..Default::default()
    };

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn reasoning_summaries_override_false_is_noop_when_model_is_false() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(false),
        ..Default::default()
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
        ..Default::default()
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
    let config = ModelsManagerConfig::default();

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn goblins_managed_model_matching_covers_current_and_namespaced_slugs() {
    let managed_slugs = [
        "gpt-5.5",
        "gpt-5.4",
        "gpt-5.4-mini",
        "gpt-5.3-codex",
        "gpt-5.3-codex-test",
        "custom/gpt-5.3-codex",
        "gpt-5.2",
        "codex-auto-review",
    ];

    for slug in managed_slugs {
        assert!(
            is_goblins_managed_model(slug),
            "expected managed slug {slug}"
        );
    }
    assert!(!is_goblins_managed_model("unknown-model"));
}

#[test]
fn fallback_model_instructions_use_current_goblins_identity() {
    let model = model_info_from_slug("custom/gpt-5.3-codex");
    let instructions = model.get_model_instructions(Some(Personality::Pragmatic));

    assert!(instructions.contains("You are a Goblin."));
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
