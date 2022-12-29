use crate::sim;

#[derive(Default)]
pub struct WeaponBannerSim {
    /// The number of wishes made against the current banner since the last 5* win.
    wish_count: u8,
    /// The number of 5* wins against the current banner since the last limited win.
    win_count: u8,
}

impl WeaponBannerSim {
    /// Determine if a roll won a 5* character.  This does not mean that the win is for the
    /// limited 5*.
    fn is_win(wish_count: u8) -> bool {
        sim::is_win(wish_count, 63, 0.007, 0.0552)
    }

    fn is_limited_win(win_count: u8) -> bool {
        // if win_count == 2, lost both 75/25s and have 2 epitomized path points, guaranteeing win
        // if fastrand::f64 < 0.375, won the 75/25 with 0 or 1 epitomized path points
        // else, lost the 75/25 with 0 or 1 epitomized path points
        win_count == 2 || fastrand::f64() < 0.375
    }
}

impl sim::BannerSim for WeaponBannerSim {
    fn wish(&mut self) -> bool {
        self.wish_count += 1;
        if WeaponBannerSim::is_win(self.wish_count) {
            self.wish_count = 0;
            if WeaponBannerSim::is_limited_win(self.win_count) {
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
}
