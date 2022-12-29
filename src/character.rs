use crate::sim;

#[derive(Default)]
pub struct CharacterBannerSim {
    /// The number of wishes made against the current banner since the last 5* win.
    wish_count: u8,
    /// The number of 5* wins against the current banner since the last limited win.
    win_count: u8,
}

impl CharacterBannerSim {
    /// Determine if a roll won a 5* character.  This does not mean that the win is for the
    /// limited 5*.
    fn is_win(wish_count: u8) -> bool {
        sim::is_win(wish_count, 74, 0.006, 0.0585)
    }

    fn is_limited_win(win_count: u8) -> bool {
        // if win_count == 1, lost the 50/50 first time, this win is guaranteed
        // if fastrand::bool(), won the 50/50
        // else, lost the 50/50
        win_count == 1 || fastrand::bool()
    }
}

impl sim::BannerSim for CharacterBannerSim {
    fn wish(&mut self) -> bool {
        self.wish_count += 1;
        if CharacterBannerSim::is_win(self.wish_count) {
            self.wish_count = 0;
            if CharacterBannerSim::is_limited_win(self.win_count) {
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
