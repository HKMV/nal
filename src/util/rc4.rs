pub struct RC4 {
    state: [u8; 256],
    i: usize,
    j: usize,
}

impl RC4 {
    pub fn new(key: &[u8]) -> RC4 {
        let mut state = [0; 256];
        for i in 0..256 {
            state[i] = i as u8;
        }

        let mut j = 0;
        for i in 0..256 {
            j = (j + state[i] as usize + key[i % key.len()] as usize) % 256;
            state.swap(i, j);
        }

        RC4 { state, i: 0, j: 0 }
    }

    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            self.i = (self.i + 1) % 256;
            self.j = (self.j + self.state[self.i] as usize) % 256;
            self.state.swap(self.i, self.j);
            let k = self.state[(self.state[self.i] as usize + self.state[self.j] as usize) % 256];
            *byte ^= k;
        }
        /*let mut i = self.i;
        let mut j = self.j;
        for byte in data.iter_mut() {
            i = (i + 1) % 256;
            j = (j + self.state[i] as usize) % 256;
            self.state.swap(i, j);
            let t = (self.state[i] as usize + self.state[j] as usize) % 256;
            *byte ^= self.state[t];
        }
        self.i = i;
        self.j = j;*/
    }
}
