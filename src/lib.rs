#[macro_export]
macro_rules! init {
    ($index_name:ident, $index_table:ident, $index_set:ident) => {
        use std::collections::hash_set;
        use std::collections::HashSet;
        use std::ops::{Index, IndexMut};
        use std::slice;

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $index_name(usize);

        pub struct $index_table<T>(Vec<T>);

        pub struct $index_set(HashSet<$index_name>);

        impl<T> $index_table<T> {
            pub fn new() -> Self {
                let v = Vec::new();
                $index_table(v)
            }

            pub fn iter(&self) -> slice::Iter<'_, T> {
                self.0.iter()
            }

            pub fn add(&mut self, value: T) -> $index_name {
                let idx = self.0.len();
                self.0.push(value);
                $index_name(idx)
            }

            pub fn remove(&mut self, index: $index_name) -> T {
                self.0.remove(index.0)
            }
        }

        impl<T> Index<$index_name> for $index_table<T> {
            type Output = T;
            fn index(&self, index: $index_name) -> &T {
                &self.0[index.0]
            }
        }

        impl<T> IndexMut<$index_name> for $index_table<T> {
            fn index_mut(&mut self, index: $index_name) -> &mut T {
                &mut self.0[index.0]
            }
        }

        impl<T> IntoIterator for $index_table<T> {
            type Item = T;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl $index_set {
            pub fn new() -> Self {
                let s = HashSet::new();
                $index_set(s)
            }

            pub fn iter(&self) -> hash_set::Iter<'_, $index_name> {
                self.0.iter()
            }

            pub fn add(&mut self, value: $index_name) -> bool {
                self.0.insert(value)
            }

            pub fn exists(&self, value: &$index_name) -> bool {
                self.0.contains(value)
            }

            pub fn remove(&mut self, value: &$index_name) -> bool {
                self.0.remove(value)
            }
        }

        impl IntoIterator for $index_set {
            type Item = $index_name;
            type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

init!(Toto, Totos, TotosSet);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_access() {
        let mut t: Totos<u32> = Totos::new();
        let i1 = t.add(1);
        let i2 = t.add(2);
        let i3 = t.add(3);
        assert_eq!(t[i1], 1);
        assert_eq!(t[i2], 2);
        assert_eq!(t[i3], 3);
    }

    #[test]
    fn table_modifs() {
        let mut t: Totos<u32> = Totos::new();
        let i1 = t.add(1);
        t[i1] = 4;
        assert_eq!(t[i1], 4);
    }

    #[test]
    fn table_iter() {
        let mut t: Totos<u32> = Totos::new();
        t.add(1);
        t.add(2);
        t.add(3);
        let mut iter = t.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn set_add() {
        let mut t: Totos<u32> = Totos::new();
        let i1 = t.add(1);
        let i2 = t.add(2);
        let i3 = t.add(3);
        let mut s = TotosSet::new();
        s.add(i1);
        s.add(i2);
        assert!(s.exists(&i1));
        assert!(s.exists(&i2));
        assert!(!s.exists(&i3));
    }
}
