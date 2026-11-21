fn clienttweaks_interact(
    game: &mut Game<'_>,
    key: InteractionKey,
    hand: Hand,
    swing_hand: &mut bool,
    canceled: &mut bool,
) {
    crate::game::umbx_clienttweaks::interaction(game, key, hand, swing_hand, canceled);
}
