// Client Tweaks' game-side logic, injected into
// `crates/horizon-client/src/game.rs` as `crate::game::umbx_clienttweaks`.
// The mixin helpers under `src/mixins/` forward to `input`, `render_hand` and
// `interaction` here. Every setting is re-read on each call, so toggles in
// Options > Mods > Client Tweaks apply live without a restart.
//
// A port of the client-side subset of ClientTweaks (Fabric, BlayTheNinth),
// reimplemented against Horizon's game internals, not a Java translation.

use crate::game::Game;

/// The mod's id, also its settings namespace.
const MOD: &str = "clienttweaks";
/// The hotbar/inventory slot the off hand holds (`shield::OFFHAND_SLOT`).
const OFFHAND_SLOT: usize = 40;
/// Looking this far up counts as "looking upwards" for auto-climb (xRot).
const CLIMB_PITCH: f32 = -60.0;

fn enabled(game: &Game<'_>, key: &str) -> bool {
    game.setting(MOD, key).as_deref() == Some("true")
}

fn choice(game: &Game<'_>, key: &str) -> usize {
    game.setting(MOD, key)
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Movement input (`Game::native_movement_input`, OVERWRITE)
// ---------------------------------------------------------------------------

/// Auto-climb feeds a forward press while on a climbable looking upwards;
/// reactive sprint clears the sprint input once its key is released.
pub fn input(game: &Game<'_>, call: &mut mod_api::__private::MovementInputCall<'_>) {
    if !game.playing() {
        return;
    }
    if enabled(game, "auto_climb")
        && game
            .player
            .as_ref()
            .is_some_and(|p| p.on_climbable && p.pitch <= CLIMB_PITCH)
    {
        *call.forward = 1.0;
        *call.forward_key = true;
    }
    // ClientTweaks' reactive sprint only matters for a hold-mode sprint key:
    // a toggled sprint already ends when the toggle is turned off.
    if enabled(game, "reactive_sprint")
        && !game.settings.toggle_sprint
        && !sprint_key_held(game)
    {
        *call.sprint = false;
    }
}

/// Whether the key `key.sprint` is bound to is physically down, on the
/// keyboard or on the pad. A binding Horizon can't map to a key code (a
/// mouse button, say) is left alone rather than read as released.
fn sprint_key_held(game: &Game<'_>) -> bool {
    if horizon_client::keybinds::key_code(game.settings.keys.get("key.sprint"))
        .is_some_and(|code| game.physical_keys.contains(&code))
    {
        return true;
    }
    game.pad.holds_action("key.sprint")
}

// ---------------------------------------------------------------------------
// First-person hands (`Game::native_render_hand`, HEAD)
// ---------------------------------------------------------------------------

/// Hides the offhand item or the empty main-hand arm by cancelling the
/// default hand geometry. `render.stack` is absent for the empty hands.
pub fn render_hand(
    game: &Game<'_>,
    render: &mod_api::HandRender<'_>,
    canceled: &mut bool,
) {
    if *canceled || !game.playing() {
        return;
    }
    match render.hand {
        mod_api::Hand::Main => {
            if enabled(game, "hide_empty_main_hand") && render.stack.is_none() {
                *canceled = true;
            }
        }
        mod_api::Hand::Off => {
            *canceled = offhand_hidden(game, render);
        }
    }
}

fn offhand_hidden(game: &Game<'_>, render: &mod_api::HandRender<'_>) -> bool {
    let holding = render.stack.is_some();
    let shield = game
        .offhand
        .as_ref()
        .is_some_and(|s| is_shield(game, s));
    if shield && shield_hidden(game) {
        return true;
    }
    match choice(game, "hide_offhand") {
        1 => holding,
        2 => holding && !offhand_usable(game),
        _ => false,
    }
}

/// The `hide_shield` modes: hide an offhand shield unless it is raised, or
/// unless the main hand holds a weapon.
fn shield_hidden(game: &Game<'_>) -> bool {
    match choice(game, "hide_shield") {
        1 => !game.using.is_some_and(|(slot, _)| slot == OFFHAND_SLOT),
        2 => !main_is_weapon(game),
        _ => false,
    }
}

fn main_is_weapon(game: &Game<'_>) -> bool {
    main_stack(game)
        .and_then(|s| game.res.items().get(&s.item))
        .is_some_and(|i| i.weapon_damage.is_some())
}

/// Whether the offhand stack would do something on a use: it places a block,
/// blocks attacks, is eaten or consumed, equips, opens or is thrown. The
/// heuristic covers vanilla's use items plus anything a mod item declares.
fn offhand_usable(game: &Game<'_>) -> bool {
    let Some(stack) = game.offhand.as_ref() else {
        return false;
    };
    let info = game.res.items().get(&stack.item);
    if stack.carries_component(info, "minecraft:food")
        || stack.carries_component(info, "minecraft:consumable")
        || stack.carries_component(info, "minecraft:blocks_attacks")
    {
        return true;
    }
    let Some(info) = info else {
        return false;
    };
    info.block.is_some()
        || info.blocks_attacks.is_some()
        || info.food.is_some()
        || info.consumable.is_some()
        || info.kinetic_timing.is_some()
        || info.equip_slot.is_some()
        || info.death_protection.is_some()
        || info.glider
        || info.bundle
        || usable_item_id(&stack.item)
}

/// Items with a use action the components above don't name: thrown or
/// projectile items, placed entities, opened books and the misc actives.
fn usable_item_id(item: &str) -> bool {
    const IDS: &[&str] = &[
        "minecraft:bow",
        "minecraft:crossbow",
        "minecraft:trident",
        "minecraft:fishing_rod",
        "minecraft:ender_pearl",
        "minecraft:snowball",
        "minecraft:egg",
        "minecraft:blue_egg",
        "minecraft:brown_egg",
        "minecraft:fire_charge",
        "minecraft:wind_charge",
        "minecraft:splash_potion",
        "minecraft:lingering_potion",
        "minecraft:experience_bottle",
        "minecraft:ender_eye",
        "minecraft:firework_rocket",
        "minecraft:goat_horn",
        "minecraft:spyglass",
        "minecraft:brush",
        "minecraft:flint_and_steel",
        "minecraft:shears",
        "minecraft:lead",
        "minecraft:name_tag",
        "minecraft:armor_stand",
        "minecraft:item_frame",
        "minecraft:glow_item_frame",
        "minecraft:painting",
        "minecraft:end_crystal",
        "minecraft:bone_meal",
        "minecraft:glass_bottle",
        "minecraft:honey_bottle",
        "minecraft:dragon_breath",
        "minecraft:writable_book",
        "minecraft:written_book",
        "minecraft:map",
        "minecraft:empty_map",
        "minecraft:filled_map",
        "minecraft:knowledge_book",
        "minecraft:saddle",
        "minecraft:totem_of_undying",
    ];
    if IDS.contains(&item) {
        return true;
    }
    // Buckets, boats, minecarts, spawn eggs and dyes come in families.
    let name = item.rsplit(':').next().unwrap_or(item);
    name.ends_with("_bucket")
        || name.ends_with("_boat")
        || name.ends_with("_raft")
        || name.ends_with("minecart")
        || name.ends_with("_spawn_egg")
        || name.ends_with("_dye")
}

// ---------------------------------------------------------------------------
// Interaction keys (`Game::native_interaction_key_mapping_triggered`, HEAD)
// ---------------------------------------------------------------------------

/// Vetoes uses and attacks. A cancelled use or attack still swings unless
/// `swing_hand` is cleared; a vetoed action should look like a miss, so the
/// swing goes too.
pub fn interaction(
    game: &mut Game<'_>,
    key: mod_api::InteractionKey,
    hand: mod_api::Hand,
    swing_hand: &mut bool,
    canceled: &mut bool,
) {
    if *canceled || !game.playing() {
        return;
    }
    let off = game.offhand.as_ref();
    let acting = if hand == mod_api::Hand::Off {
        off
    } else {
        main_stack(game)
    };
    let cancel = match key {
        mod_api::InteractionKey::Use => {
            (enabled(game, "mending_last_durability") && mending_save(game, acting))
                || (hand == mod_api::Hand::Off && offhand_use_blocked(game, off))
                || (hand == mod_api::Hand::Main && log_strip_blocked(game, acting))
        }
        mod_api::InteractionKey::Attack => {
            (enabled(game, "mending_last_durability") && mending_save(game, acting))
                || (enabled(game, "prevent_accidental_mining") && amethyst_blocked(game))
        }
        _ => false,
    };
    if cancel {
        *canceled = true;
        *swing_hand = false;
    }
}

fn main_stack<'a>(game: &'a Game<'_>) -> Option<&'a horizon_items::items::ItemStack> {
    game.inventory.get(game.selected).and_then(Option::as_ref)
}

/// The offhand use vetoes: food in the main hand blocks every offhand use,
/// an elytra on the ground blocks offhand fireworks, and the torch policy
/// narrows when a light-emitting block may leave the offhand.
fn offhand_use_blocked(
    game: &Game<'_>,
    off: Option<&horizon_items::items::ItemStack>,
) -> bool {
    let Some(off) = off else {
        return false;
    };
    let main = main_stack(game);
    if enabled(game, "no_offhand_use_with_food")
        && main.is_some_and(|m| is_food(game, m))
    {
        return true;
    }
    if enabled(game, "no_offhand_fireworks")
        && is_firework(game, off)
        && grounded_elytra(game)
    {
        return true;
    }
    if !is_light_emitting(game, off) {
        return false;
    }
    if enabled(game, "keep_last_torch") && off.count == 1 {
        return true;
    }
    match choice(game, "offhand_torch") {
        1 => true,
        2 => main.is_some_and(|m| is_block_item(game, m)),
        3 => main.is_some_and(|m| is_food(game, m)),
        4 => main.is_none_or(|m| m.count == 0),
        5 => !main.is_some_and(|m| is_tool(game, m)),
        _ => false,
    }
}

/// An elytra is worn but the wearer is on the ground and not gliding.
fn grounded_elytra(game: &Game<'_>) -> bool {
    game.player
        .as_ref()
        .is_some_and(|p| p.glider && !p.fall_flying && p.ground.on_ground)
}

/// `log_stripping`: `disabled` vetoes every strip, `requires_sneak` only
/// while not sneaking. What the axe's `BlockTransformer` would produce
/// decides whether this click strips, so copper scraping and wax removal
/// stay untouched.
fn log_strip_blocked(
    game: &Game<'_>,
    acting: Option<&horizon_items::items::ItemStack>,
) -> bool {
    match choice(game, "log_stripping") {
        1 => {}
        2 => {
            if game
                .player
                .as_ref()
                .is_some_and(|p| p.shift_key_down)
            {
                return false;
            }
        }
        _ => return false,
    }
    let Some(stack) = acting else {
        return false;
    };
    let Some(hit) = game.pick() else {
        return false;
    };
    let Some((new, ..)) = game.block_transform(stack, &hit) else {
        return false;
    };
    game.res.facts().get(new).name.contains("stripped_")
}

/// `prevent_accidental_mining`: budding amethyst and its buds are nearly
/// irreplaceable, so they only break while sneaking.
fn amethyst_blocked(game: &Game<'_>) -> bool {
    if game
        .player
        .as_ref()
        .is_some_and(|p| p.shift_key_down)
    {
        return false;
    }
    let Some(hit) = game.pick() else {
        return false;
    };
    let state = game.world.block(hit.pos[0], hit.pos[1], hit.pos[2]);
    matches!(
        &*game.res.facts().get(state).name,
        "minecraft:budding_amethyst"
            | "minecraft:amethyst_cluster"
            | "minecraft:large_amethyst_bud"
            | "minecraft:medium_amethyst_bud"
            | "minecraft:small_amethyst_bud"
    )
}

/// `mending_last_durability`: a Mending tool mends itself, so a use or
/// attack on its last durability point is refused rather than broken.
fn mending_save(
    game: &Game<'_>,
    stack: Option<&horizon_items::items::ItemStack>,
) -> bool {
    let Some(stack) = stack else {
        return false;
    };
    if stack.enchantment_level("minecraft:mending") == 0 {
        return false;
    }
    let max = stack
        .data()
        .and_then(|d| d.other("minecraft:max_damage"))
        .and_then(|tag| match tag {
            horizon_core::nbt::Tag::Int(v) => u32::try_from(*v).ok(),
            _ => None,
        })
        .unwrap_or_else(|| {
            game.res
                .items()
                .get(&stack.item)
                .map_or(0, |i| i.max_damage)
        });
    max > 0 && stack.damage >= max.saturating_sub(1)
}

// ---------------------------------------------------------------------------
// Item classification
// ---------------------------------------------------------------------------

fn is_food(game: &Game<'_>, stack: &horizon_items::items::ItemStack) -> bool {
    stack.carries_component(game.res.items().get(&stack.item), "minecraft:food")
}

fn is_shield(game: &Game<'_>, stack: &horizon_items::items::ItemStack) -> bool {
    stack.carries_component(game.res.items().get(&stack.item), "minecraft:blocks_attacks")
}

fn is_block_item(game: &Game<'_>, stack: &horizon_items::items::ItemStack) -> bool {
    game.res
        .items()
        .get(&stack.item)
        .is_some_and(|i| i.block.is_some())
}

fn is_tool(game: &Game<'_>, stack: &horizon_items::items::ItemStack) -> bool {
    game.res.items().get(&stack.item).is_some_and(|i| {
        !i.tool.is_empty() || i.block_transformer.is_some() || i.weapon_damage.is_some()
    })
}

fn is_firework(game: &Game<'_>, stack: &horizon_items::items::ItemStack) -> bool {
    game.res
        .items()
        .get(&stack.item)
        .is_some_and(|i| i.is("minecraft:firework_rocket"))
}

/// A torch for the torch rules: any block item whose block emits light, as
/// upstream's "light emitting block" check defines it.
fn is_light_emitting(
    game: &Game<'_>,
    stack: &horizon_items::items::ItemStack,
) -> bool {
    let Some(state) = game
        .res
        .items()
        .get(&stack.item)
        .and_then(|i| i.block)
    else {
        return false;
    };
    game.res.facts().get(state).emission > 0
}
