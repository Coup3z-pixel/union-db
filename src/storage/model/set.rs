use roaring::RoaringBitmap;
use std::fmt;

/// A named set of 32-bit integers backed by a Roaring Bitmap.
///
/// This struct provides efficient storage and manipulation
/// of large sets of integers, along with a human-readable
/// name label for identification.
pub struct Set {
    /// The name of the set.
    set_name: String,

    /// The underlying RoaringBitmap storing the elements.
    bitmap: RoaringBitmap,
}

impl Set {
    /// Creates a new empty `Set` with the given name.
    ///
    /// # Arguments
    ///
    /// * `set_name` - A string slice to identify the set.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Set::new("example");
    /// ```
    pub fn new(set_name: &str) -> Self {
        Self {
            set_name: set_name.to_string(),
            bitmap: RoaringBitmap::new(),
        }
    }

    /// Creates a new `Set` from an existing `RoaringBitmap` and name.
    ///
    /// # Arguments
    ///
    /// * `set_name` - Name of the set.
    /// * `bitmap` - Reference to an existing RoaringBitmap to clone.
    ///
    /// # Examples
    ///
    /// ```
    /// let bitmap = RoaringBitmap::new();
    /// let s = Set::new_with_set("cloned_set", &bitmap);
    /// ```
    pub fn new_with_set(set_name: &str, bitmap: &RoaringBitmap) -> Self {
        Self {
            set_name: set_name.to_string(),
            bitmap: bitmap.clone(),
        }
    }

    /// Returns the name of the set.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Set::new("myset");
    /// assert_eq!(s.get_set_name(), "myset");
    /// ```
    pub fn get_set_name(&self) -> &str {
        &self.set_name
    }

    /// Inserts a value into the set.
    ///
    /// # Arguments
    ///
    /// * `val` - The 32-bit integer to insert.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// s.insert(42);
    /// assert!(s.contains(42));
    /// ```
    pub fn insert(&mut self, val: u32) {
        self.bitmap.insert(val);
    }

    /// Removes a value from the set, if present.
    ///
    /// # Arguments
    ///
    /// * `val` - The 32-bit integer to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// s.insert(42);
    /// s.remove(42);
    /// assert!(!s.contains(42));
    /// ```
    pub fn remove(&mut self, val: u32) {
        self.bitmap.remove(val);
    }

    /// Checks if a value exists in the set.
    ///
    /// # Arguments
    ///
    /// * `val` - The 32-bit integer to check.
    ///
    /// # Returns
    ///
    /// `true` if the value is present, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// s.insert(42);
    /// assert!(s.contains(42));
    /// assert!(!s.contains(100));
    /// ```
    pub fn contains(&self, val: u32) -> bool {
        self.bitmap.contains(val)
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// assert_eq!(s.len(), 0);
    /// s.insert(42);
    /// assert_eq!(s.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.bitmap.len() as usize
    }

    /// Checks whether the set is empty.
    ///
    /// # Returns
    ///
    /// `true` if the set contains no elements, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Set::new("empty");
    /// assert!(s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }

    /// Removes all elements from the set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// s.insert(42);
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.bitmap.clear();
    }

    /// Returns an iterator over all elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Set::new("numbers");
    /// s.insert(1);
    /// s.insert(2);
    /// for val in s.iter() {
    ///     println!("{}", val);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.bitmap.iter()
    }

    /// Returns a reference to the underlying `RoaringBitmap`.
    ///
    /// Useful if you want to perform advanced bitmap operations directly.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Set::new("numbers");
    /// let bitmap_ref = s.bitmap();
    /// ```
    pub fn bitmap(&self) -> &RoaringBitmap {
        &self.bitmap
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set {{ name: {}, size: {}, values: [", self.set_name, self.len())?;
        for (i, val) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        write!(f, "] }}")
    }
}
