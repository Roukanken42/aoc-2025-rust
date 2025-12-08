use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T: Eq + Hash + Clone> {
    data: HashMap<T, T>,
}

impl<T: Eq + Hash + Clone> UnionFind<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, x: &T) {
        self.data.insert(x.clone(), x.clone());
    }

    pub fn find(&mut self, x: &T) -> Option<T> {
        let group = self.data.get(x).cloned();

        let parent = match group {
            None => return None,
            Some(parent) if &parent == x => return Some(parent),
            Some(parent) => self.find(&parent),
        };

        if let Some(parent) = parent {
            self.data.insert(x.clone(), parent.clone());
            Some(parent)
        } else {
            parent
        }
    }

    pub fn union(&mut self, x: &T, y: &T) -> Option<(T, T)> {
        let x_parent = self.find(x);
        let y_parent = self.find(y);

        match (x_parent, y_parent) {
            (Some(x_parent), Some(y_parent)) => {
                self.data.insert(x_parent.clone(), y_parent.clone());
                Some((x_parent, y_parent))
            }
            _ => None,
        }
    }
}

impl<T: Eq + Hash + Clone> FromIterator<T> for UnionFind<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = iter.into_iter().map(|x| (x.clone(), x)).collect();
        UnionFind { data }
    }
}
