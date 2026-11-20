# Client Tweaks (Horizon native port)

A native Rust port of the client-side subset of
[ClientTweaks](https://github.com/TwelveIterations/ClientTweaks) (Fabric, by
BlayTheNinth) for [Horizon](https://playhorizon.dev) — Minecraft 26.3 rewritten
in Rust. Every tweak is off by default and configured per-feature in
**Options > Mods > Client Tweaks**; changes apply live, no restart needed.

## Features ported

| Tweak | Setting | What it does |
|---|---|---|
| Auto Climb Ladder | `auto_climb` | Climb ladders (and other climbable blocks) by looking upwards (pitch ≤ −60°) instead of holding a key. |
| Reactive Sprint Key | `reactive_sprint` | Sprinting stops the tick the sprint key is released. Only applies to a hold-mode sprint key; a toggled sprint is untouched. |
| Hide Offhand Item | `hide_offhand` | Hides the first-person offhand — `Always`, or `Unless usable` (keeps placeable blocks, shields, food, tools with a use action and equippables visible). |
| Hide Shield | `hide_shield` | Hides an offhand shield `Unless blocking` (it is raised) or `Unless weapon in main hand`. Combines with `hide_offhand`. |
| Hide Empty Main Hand | `hide_empty_main_hand` | Hides the bare first-person arm when the main hand holds nothing. |
| Offhand Torch | `offhand_torch` | Restricts placing light-emitting blocks (upstream's "torch" definition) from the offhand: `Never`, `Not with a block/food/an empty main hand`, or `Only with a tool in main hand`. Covers upstream's six separate torch toggles. |
| Do Not Use Last Torch | `keep_last_torch` | Never places the last stack of a light-emitting block held in the offhand. |
| No Offhand Use With Food | `no_offhand_use_with_food` | Blocks every offhand use while the main hand holds food. |
| No Offhand Fireworks With Elytra | `no_offhand_fireworks` | Blocks offhand firework rockets while an elytra is worn and the player is on the ground (not gliding). |
| Disable / Sneak Log Stripping | `log_stripping` | `Disabled` blocks stripping entirely; `Only while sneaking` requires sneak. Detects what the axe's block transformer would produce, so scraping and de-waxing copper still work. |
| Prevent Accidental Mining | `prevent_accidental_mining` | Budding amethyst, amethyst buds and clusters only break while sneaking. |
| Do Not Use Last Durability (Mending) | `mending_last_durability` | Refuses to use or attack with a Mending-enchanted item on its last durability point — Mending repairs it, so it should never break. |

## Upstream features not ported

- **Master/Music volume sliders** — Horizon already exposes vanilla's volume
  sliders; a separate mod page for them adds nothing.
- **Prioritize Experience Bar** — needs a HUD layout hook Horizon doesn't
  expose to native mods yet.
- **Hide Hands** (both) — covered piecemeal by `hide_offhand` +
  `hide_empty_main_hand`; a single "hide everything" felt redundant.
- **Hide Own Particle Effects (first/third person)** — own-entity particles
  are client-computed in Horizon; the particle hooks cover server-driven
  muffle only.
- **Disable Step Assist** — upstream disables step assist *added by other
  mods*; vanilla has no step assist to disable.
- **Berry placement requires shift / disable paving / other interaction
  guards** — kept the port small; the interaction mixin makes these easy
  follow-ups.
- **Key-binding toggles** — upstream can bind `hideOffhandItem` to a key;
  Horizon native settings are toggled from the settings page instead.

## Architecture

- `src/lib.rs` — the mod crate: registers the settings specs.
- `src/logic.rs` — injected into the game as `crate::game::umbx_clienttweaks`;
  holds all tweak logic and re-reads settings on every event.
- `src/mixins/` — thin helpers:
  - `input.rs` OVERWRITEs `native_movement_input` (auto climb, reactive sprint),
  - `hand.rs` HEADs `native_render_hand` (hidden hands/shield),
  - `interact.rs` HEADs `native_interaction_key_mapping_triggered`
    (torch/offhand/strip/mining/durability vetoes).

## Build

Builds inside a Horizon checkout via the launcher's `native-build`, like any
native mod. Pinned against game `26.3`, Horizon
`f00125f61f794aaa3d4e950d301f353b35b6ee3d`, mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.
