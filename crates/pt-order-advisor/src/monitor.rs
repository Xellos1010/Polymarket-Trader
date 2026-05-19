use crate::advisor::PlacementMode;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonitorConfig {
    pub adverse_ticks_to_arm: usize,
    pub window_secs: u64,
}

impl Default for MonitorConfig {
    fn default() -> Self { Self { adverse_ticks_to_arm: 3, window_secs: 60 } }
}

#[derive(Debug, Default)]
pub struct ProfitProtectionMonitor {
    cfg: MonitorConfig,
    position_direction: i8,
    pub consecutive_adverse: usize,
    gate1_armed: bool,
}

impl ProfitProtectionMonitor {
    pub fn new(cfg: MonitorConfig) -> Self { Self { cfg, ..Default::default() } }

    pub fn on_position_opened(&mut self, direction: i8) {
        self.position_direction = direction;
        self.consecutive_adverse = 0;
        self.gate1_armed = false;
    }

    pub fn on_position_closed(&mut self) {
        self.position_direction = 0;
        self.consecutive_adverse = 0;
        self.gate1_armed = false;
    }

    pub fn on_tick(&mut self, price_direction: i8) {
        if self.position_direction == 0 { return; }
        let adverse = price_direction != 0 && price_direction != self.position_direction;
        if adverse {
            self.consecutive_adverse += 1;
        } else {
            self.consecutive_adverse = 0;
        }
        if self.consecutive_adverse >= self.cfg.adverse_ticks_to_arm {
            self.gate1_armed = true;
        }
    }

    pub fn check(&self, signal_direction: i8) -> PlacementMode {
        if self.position_direction == 0 { return PlacementMode::Maker; }
        if !self.gate1_armed { return PlacementMode::Maker; }
        let signal_flipped = signal_direction != 0
            && signal_direction != self.position_direction;
        if signal_flipped { PlacementMode::Taker } else { PlacementMode::Maker }
    }

    pub fn is_armed(&self) -> bool { self.gate1_armed }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_position_always_maker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig::default());
        m.on_tick(-1); m.on_tick(-1); m.on_tick(-1);
        assert_eq!(m.check(-1), PlacementMode::Maker);
    }

    #[test]
    fn gate1_not_armed_until_n_consecutive() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 3, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1); m.on_tick(-1);
        assert!(!m.is_armed());
        m.on_tick(-1);
        assert!(m.is_armed());
    }

    #[test]
    fn favorable_tick_resets_consecutive() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig::default());
        m.on_position_opened(1);
        m.on_tick(-1); m.on_tick(-1);
        m.on_tick(1);
        assert_eq!(m.consecutive_adverse, 0);
        assert!(!m.is_armed());
    }

    #[test]
    fn both_gates_open_recommends_taker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1); m.on_tick(-1);
        assert_eq!(m.check(-1), PlacementMode::Taker);
    }

    #[test]
    fn gate1_armed_but_signal_agrees_stays_maker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1); m.on_tick(-1);
        assert_eq!(m.check(1), PlacementMode::Maker);
    }

    #[test]
    fn position_close_resets_monitor() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1); m.on_tick(-1);
        assert!(m.is_armed());
        m.on_position_closed();
        assert!(!m.is_armed());
        assert_eq!(m.check(-1), PlacementMode::Maker);
    }

    #[test]
    fn short_position_adverse_tick_is_price_rise() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(-1);
        m.on_tick(1); m.on_tick(1);
        assert!(m.is_armed());
    }
}
