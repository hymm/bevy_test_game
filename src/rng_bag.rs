use rand::seq::IteratorRandom;
pub struct RngBag<T> {
    items: Vec<T>,
    original_items: Vec<T>,
}

impl<T: Copy> RngBag<T> {
    pub fn new(items: Vec<T>) -> RngBag<T> {
        RngBag {
            items: items.clone(),
            original_items: items.clone(),
        }
    }

    pub fn get(&mut self) -> T {
        let mut rng = rand::thread_rng();
        if self.items.len() == 0 {
            self.items = self.original_items.clone();
        }

        let (n, _) = self.items.iter().enumerate().choose(&mut rng).unwrap();
        let item = self.items.remove(n);

        item
    }
}
