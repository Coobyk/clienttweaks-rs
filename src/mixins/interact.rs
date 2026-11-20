fn clienttweaks_interact(
    game: &mut Game<'_>,
    key: InteractionKey,
    hand: Hand,
    attempt: InteractionAttempt,
    swing_hand: &mut bool,
    canceled: &mut bool,
) {
    crate::game::umbx_clienttweaks::interaction(game, key, hand, attempt, swing_hand, canceled);
}
