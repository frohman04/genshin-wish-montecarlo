pub trait SimParams {
    /// Get the (soft_pity, max_pity, base_win_pct, incr_win_pct) for the banner.  max_pity is the
    /// most wishes that are required to get the desired limited 5* item.
    fn get_win_params(&self) -> (u8, u8, f64, f64);

    /// Determine if a 5* win resulted in the desired limited 5* item.
    fn is_limited_win(&self, win_count: u8) -> bool;
}

pub struct BannerSim {
    /// The parameters to use when running the simulation
    params: Box<dyn SimParams>,
    soft_pity: u8,
    max_pity: u8,
    base_win_pct: f64,
    incr_win_pct: f64,
    /// The number of wishes made against the current banner since the last 5* win.
    wish_count: u8,
    /// The number of 5* wins against the current banner since the last limited win.
    win_count: u8,
}

impl BannerSim {
    pub fn new(params: Box<dyn SimParams>) -> BannerSim {
        let (soft_pity, hard_pity, base_win_pct, incr_win_pct) = params.get_win_params();
        BannerSim {
            params,
            soft_pity,
            max_pity: hard_pity,
            base_win_pct,
            incr_win_pct,
            wish_count: 0,
            win_count: 0,
        }
    }

    /// Wish on the banner.  Will return true when the targeted item is won.
    pub fn wish(&mut self) -> bool {
        self.wish_count += 1;
        if BannerSim::is_win(
            self.wish_count,
            self.soft_pity,
            self.base_win_pct,
            self.incr_win_pct,
        ) {
            self.wish_count = 0;
            if self.params.is_limited_win(self.win_count) {
                self.win_count = 0;
                true
            } else {
                self.win_count += 1;
                false
            }
        } else {
            false
        }
    }

    pub fn get_max_pity(&self) -> u8 {
        self.max_pity
    }

    /// Determine if a given wish is a winner.  Wishes prior to soft_pity (exclusive) will win at
    /// base_win_pct rate.  After hitting soft_pity, each successive wish will gan incr_win_pct
    /// additional chance at winning, capping at 100% chance at hard pity.
    fn is_win(wish_count: u8, soft_pity: u8, base_win_pct: f64, incr_win_pct: f64) -> bool {
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
}
