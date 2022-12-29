use crate::sim;

#[derive(Default)]
pub struct CharacterBannerSimParams {}

impl CharacterBannerSimParams {
    pub fn new_sim() -> sim::BannerSim {
        #[allow(clippy::box_default)]
        sim::BannerSim::new(Box::new(CharacterBannerSimParams::default()))
    }
}

impl sim::SimParams for CharacterBannerSimParams {
    fn get_win_params(&self) -> (u8, f64, f64) {
        (74, 0.006, 0.0585)
    }

    fn is_limited_win(&self, win_count: u8) -> bool {
        // if win_count == 1, lost the 50/50 first time, this win is guaranteed
        // if fastrand::bool(), won the 50/50
        // else, lost the 50/50
        win_count == 1 || fastrand::bool()
    }
}
