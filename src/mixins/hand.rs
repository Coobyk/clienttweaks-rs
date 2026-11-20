fn clienttweaks_hand(game: &Game<'_>, render: &mut HandRender<'_>, canceled: &mut bool) {
    crate::game::umbx_clienttweaks::render_hand(game, render, canceled);
}
