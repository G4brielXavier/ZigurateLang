pub struct Eyes<'a> {
    pub src: &'a [u8],
    pub i: usize,
    pub ln: u16,
    pub col: u8
}

impl<'a> Eyes<'a> {

    pub fn open(src: &'a [u8]) -> Self {
        Self {
            src,
            i: 0,
            ln: 1,
            col: 0
        }
    }

    pub fn look(&self) -> Option<u8> {

        if self.i < self.src.len() {
            Some(self.src[self.i])
        } else {
            None
        }

    }


    pub fn look_next(&self) -> Option<u8> {

        if self.i + 1 < self.src.len() {
            Some(self.src[self.i + 1])
        } else {
            None
        }

    }

    pub fn blink(&mut self) -> Option<u8> {

        let char = self.look();

        if let Some(char) = char {
            if char == b'\n' {
                self.ln += 1;
                self.col = 0;
            } else {
                self.col += 1
            }

            self.i += 1;

            Some(char)

        } else {
            None
        }

    }

}