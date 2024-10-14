#[derive(Debug, Clone)]
pub struct Tape {
    data: Vec<u8>,
    data_offset: isize,
    head: isize,
}
impl Tape {
    pub fn new() -> Tape {
        Tape {
            data: Vec::new(),
            data_offset: 0,
            head: 0,
        }
    }
    pub fn move_right(&mut self) {
        self.head += 1;
    }
    pub fn move_left(&mut self) {
        self.head -= 1;
    }
    pub fn increment(&mut self) {
        self.set(self.head, self.get(self.head).wrapping_add(1));
    }
    pub fn decrement(&mut self) {
        self.set(self.head, self.get(self.head).wrapping_sub(1));
    }
    pub fn read(&self) -> u8 {
        self.get(self.head)
    }
    pub fn write(&mut self, val: u8) {
        self.set(self.head, val)
    }

    pub fn get(&self, index: isize) -> u8 {
        let real_index = self.data_offset + index;

        if (0..self.data.len()).contains(&(real_index as usize)) {
            self.data[real_index as usize]
        } else {
            0
        }
    }
    pub fn set(&mut self, index: isize, val: u8) {
        let mut real_index = self.data_offset + index;

        while real_index.is_negative() {
            self.data.insert(0, 0);
            self.data_offset += 1;
            real_index += 1;
        }

        while real_index as usize >= self.data.len() {
            self.data.push(0);
        }

        self.data[real_index as usize] = val;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn set_many(values: Vec<(i16, u8)>) {
            let mut tape = Tape::new();
            let mut tape_copy = vec![0; 65536];

            for (index, value) in values {
                tape.set(index as isize, value);
                tape_copy[index as u16 as usize] = value;
            }

            let tape = tape;
            let tape_copy = tape_copy;

            for i in i16::MIN..=i16::MAX {
                assert_eq!(tape.get(i as isize), tape_copy[i as u16 as usize]);
            }
        }
    }
}
