pub trait BannerSim {
    /// Wish on the banner.  Will return true when the targeted item is won.
    fn wish(&mut self) -> bool;
}

/// Determine if a given wish is a winner.  Wishes prior to soft_pity (exclusive) will win at
/// base_win_pct rate.  After hitting soft_pity, each successive wish will gan incr_win_pct
/// additional chance at winning, capping at 100% chance at hard pity.
pub fn is_win(wish_count: u8, soft_pity: u8, base_win_pct: f64, incr_win_pct: f64) -> bool {
    let pct_win = if wish_count < soft_pity {
        base_win_pct
    } else {
        // this is slightly >1 for hard pity, but that's ok because the RNG will not generate
        // a value greater than 1
        base_win_pct + incr_win_pct * ((wish_count - (soft_pity - 1)) as f64)
    };

    // need eq to ensure that hard pity generates a win
    fastrand::f64() <= pct_win
}
