pub trait BannerSim {
    /// Wish on the banner.  Will return true when the targeted item is won.
    fn wish(&mut self) -> bool;
}
