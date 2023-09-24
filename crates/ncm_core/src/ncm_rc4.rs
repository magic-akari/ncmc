#[derive(Debug, Clone)]
pub(crate) struct NcmRc4 {
    state: [u8; 256],
}

impl NcmRc4 {
    pub fn new(key: &[u8]) -> Self {
        let mut rc4 = NcmRc4 { state: [0; 256] };

        rc4.ncm_prga(&Self::ksa(key));

        rc4
    }

    fn ksa(key: &[u8]) -> [u8; 256] {
        let mut state = [0; 256];

        state.iter_mut().enumerate().for_each(|(i, x)| {
            *x = i as u8;
        });

        let key_iter = key.iter().cycle();

        let mut j = 0u8;

        (0..=255).zip(key_iter).for_each(|(i, k)| {
            j = j.wrapping_add(state[i]).wrapping_add(*k);

            state.swap(i, j.into());
        });

        state
    }

    fn ncm_prga(&mut self, state: &[u8; 256]) {
        (0..=255u8).for_each(|i| {
            let key1 = i.wrapping_add(1);
            let key2 = key1.wrapping_add(state[key1 as usize]);
            let index = state[key1 as usize].wrapping_add(state[key2 as usize]);
            self.state[i as usize] = state[index as usize];
        });
    }
}

impl IntoIterator for NcmRc4 {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 256>;

    fn into_iter(self) -> Self::IntoIter {
        self.state.into_iter()
    }
}
