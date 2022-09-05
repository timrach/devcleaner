use num_traits::ToPrimitive;
use tui::widgets::ListState;

pub struct StatefulList<T>
where
    T: PartialEq,
{
    pub state: ListState,
    pub marked_items: Vec<usize>,
    pub items: Vec<T>,
}

impl<T> StatefulList<T>
where
    T: PartialEq,
{
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            marked_items: vec![],
            items,
        }
    }

    pub fn toggle_all(&mut self) {
        if self.marked_items.is_empty() {
            let mut indices: Vec<usize> = (0..self.items.len()).collect();
            self.marked_items.append(&mut indices);
        } else {
            self.marked_items.clear();
        }
    }

    pub fn is_marked(&self, item: &T) -> bool {
        if let Some(idx) = self.items.iter().position(|x| x == item) {
            return self.marked_items.contains(&idx);
        }
        false
    }

    pub fn toggle_current(&mut self) {
        if let Some(idx) = self.state.selected() {
            if self.marked_items.contains(&idx) {
                if let Some(pos) = self.marked_items.iter().position(|x| x == &idx) {
                    self.marked_items.remove(pos);
                }
            } else {
                self.marked_items.push(idx);
            }
        }
    }

    pub fn shift_selection(&mut self, offset: i32) {
        let len = self.items.len().to_i32().unwrap_or_default();
        if len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i.to_i32().unwrap_or_default() + offset) % len,
            None => 0,
        };

        let i = if i < 0 { len + i } else { i };

        self.state.select(Some(i.to_usize().unwrap_or(0)));
    }
}
