use super::{PrimitiveProperty, Property};

/// A random number generator for gacha systems using xorshift32 algorithm.
pub struct GachaRandom {
    state: PrimitiveProperty<u32>,
}

impl GachaRandom {
    /// Creates a new GachaRandom with the given seed.
    /// If seed is 0, a default non-zero seed is used to avoid poor RNG behavior.
    pub fn new(seed: u32) -> Self {
        let initial_state = if seed == 0 { 0x12345678 } else { seed };
        GachaRandom {
            state: initial_state.into(), // Use into to create PrimitiveProperty<u32>
        }
    }

    /// Generates a random u32 value in the range [0, max).
    /// Returns 0 if max is 0 to avoid division by zero.
    pub fn rand(&mut self, max: u32) -> u32 {
        let next = self.next_state();
        if max == 0 {
            0
        } else {
            ((next as u64) % (max as u64)) as u32
        }
    }

    /// Generates a random u32 in the range [min, max).
    /// Returns min if max <= min.
    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        if max <= min {
            return min;
        }
        min.wrapping_add(self.rand(max - min))
    }

    /// Generates a random f32 in the range [0.0, 1.0).
    pub fn rand_float(&mut self) -> f32 {
        let value = self.next_state();
        // Normalize to [0.0, 1.0) using u32::MAX
        (value as f32) / (u32::MAX as f32 + 1.0)
    }

    /// Sets a new seed for the RNG.
    /// If seed is 0, a default non-zero seed is used.
    pub fn set_seed(&mut self, seed: u32) {
        let new_state = if seed == 0 { 0x12345678 } else { seed };
        self.state.set(new_state);
    }

    /// Returns the current seed.
    pub fn seed(&self) -> u32 {
        self.state.get()
    }

    /// Internal method to generate the next state using xorshift32.
    fn next_state(&mut self) -> u32 {
        let mut state = self.state.get();
        state ^= state.wrapping_shl(13);
        state ^= state.wrapping_shr(17);
        state ^= state.wrapping_shl(5);
        self.state.set(state);
        state
    }
}

impl Property for GachaRandom {
    fn is_changed(&self) -> bool {
        self.state.is_changed()
    }

    fn reset_changed_state(&mut self) {
        self.state.reset_changed_state();
    }
}

impl From<u32> for GachaRandom {
    fn from(seed: u32) -> Self {
        GachaRandom::new(seed)
    }
}