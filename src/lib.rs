//! Client Tweaks' own crate: the settings page (Options > Mods > Client
//! Tweaks). The tweaks themselves run in the game-side module injected as
//! `crate::game::umbx_clienttweaks` (see `src/logic.rs`); it re-reads these
//! settings on every event so page changes apply live.

use mod_api::{Hooks, Registrar, RegistrationContext, SettingSpec};

#[derive(Default)]
struct Mod;
#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, reg: &mut dyn Registrar, _: &RegistrationContext) {
        for spec in [
            SettingSpec::toggle("auto_climb", "clienttweaks.setting.auto_climb", false),
            SettingSpec::toggle(
                "reactive_sprint",
                "clienttweaks.setting.reactive_sprint",
                false,
            ),
            SettingSpec::choice(
                "hide_offhand",
                "clienttweaks.setting.hide_offhand",
                [
                    "clienttweaks.choice.off",
                    "clienttweaks.choice.always",
                    "clienttweaks.choice.unless_usable",
                ],
                0,
            ),
            SettingSpec::choice(
                "hide_shield",
                "clienttweaks.setting.hide_shield",
                [
                    "clienttweaks.choice.off",
                    "clienttweaks.choice.unless_blocking",
                    "clienttweaks.choice.unless_weapon",
                ],
                0,
            ),
            SettingSpec::toggle(
                "hide_empty_main_hand",
                "clienttweaks.setting.hide_empty_main_hand",
                false,
            ),
            SettingSpec::choice(
                "offhand_torch",
                "clienttweaks.setting.offhand_torch",
                [
                    "clienttweaks.choice.off",
                    "clienttweaks.choice.never",
                    "clienttweaks.choice.no_block_in_main",
                    "clienttweaks.choice.no_food_in_main",
                    "clienttweaks.choice.no_empty_main",
                    "clienttweaks.choice.tool_only",
                ],
                0,
            ),
            SettingSpec::toggle(
                "keep_last_torch",
                "clienttweaks.setting.keep_last_torch",
                false,
            ),
            SettingSpec::toggle(
                "no_offhand_use_with_food",
                "clienttweaks.setting.no_offhand_use_with_food",
                false,
            ),
            SettingSpec::toggle(
                "no_offhand_fireworks",
                "clienttweaks.setting.no_offhand_fireworks",
                false,
            ),
            SettingSpec::choice(
                "log_stripping",
                "clienttweaks.setting.log_stripping",
                [
                    "clienttweaks.choice.off",
                    "clienttweaks.choice.disabled",
                    "clienttweaks.choice.requires_sneak",
                ],
                0,
            ),
            SettingSpec::toggle(
                "prevent_accidental_mining",
                "clienttweaks.setting.prevent_accidental_mining",
                false,
            ),
            SettingSpec::toggle(
                "mending_last_durability",
                "clienttweaks.setting.mending_last_durability",
                false,
            ),
        ] {
            let tip = format!("{}.tooltip", spec.label);
            let spec = spec.tooltip(tip);
            let _ = reg.register_setting(spec);
        }
    }
}
