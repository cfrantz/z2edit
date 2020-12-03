
pub struct UndoStack<T> {
    maxlen: isize,
    index: isize,
    stack: Vec<T>,
}

impl<T> UndoStack<T> {
    pub fn new(maxlen: usize) -> UndoStack<T> {
        UndoStack {
            maxlen: maxlen as isize,
            index: -1,
            stack: Vec::<T>::with_capacity(maxlen),
        }
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.index = -1;
    }

    pub fn reset(&mut self, value: T) {
        self.clear();
        self.push(value);
    }

    pub fn push(&mut self, value: T) {
        let len = self.stack.len() as isize - 1;
        if self.index == len {
            // Empty stack or index at the last spot.
            if len == self.maxlen {
                // If we're also at the maxlen, delete the first element
                self.stack.remove(0);
                self.index -= 1;
            }
        } else {
            self.stack.truncate((self.index + 1) as usize);
        }
        self.stack.push(value);
        self.index += 1;
        info!("UndoStack::push index = {}", self.index);
    }

    pub fn undo(&mut self) -> Option<&T> {
        if self.index >= 0 {
            self.index -= 1;
            info!("UndoStack::undo index = {}", self.index);
            self.stack.get(self.index as usize)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&T> {
        let len = self.stack.len() as isize - 1;
        if self.index < len {
            self.index += 1;
            info!("UndoStack::redo index = {}", self.index);
            self.stack.get(self.index as usize)
        } else {
            None
        }
    }
}
