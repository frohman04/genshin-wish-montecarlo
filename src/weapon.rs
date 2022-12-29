use crate::sim;

#[derive(Default)]
pub struct WeaponBannerSimParams {}

impl WeaponBannerSimParams {
    pub fn new_sim() -> sim::BannerSim {
        #[allow(clippy::box_default)]
        sim::BannerSim::new(Box::new(WeaponBannerSimParams::default()))
    }
}

impl sim::SimParams for WeaponBannerSimParams {
    fn get_win_params(&self) -> (u8, f64, f64) {
        (63, 0.007, 0.0552)
    }

    fn is_limited_win(&self, win_count: u8) -> bool {
        // if win_count == 2, lost both 75/25s and have 2 epitomized path points, guaranteeing win
        // if fastrand::f64 < 0.375, won the 75/25 with 0 or 1 epitomized path points
        // else, lost the 75/25 with 0 or 1 epitomized path points
        win_count == 2 || fastrand::f64() < 0.375
    }
}
