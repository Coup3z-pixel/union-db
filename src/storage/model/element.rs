pub trait Element {
    fn to_element(&self) -> Result<u32, String>;
}

impl Element for u32 {
    fn to_element(&self) -> Result<u32, String> { Ok(*self) } 
}

impl Element for u16 {
    fn to_element(&self) -> Result<u32, String> { Ok(*self as u32) }
}

impl Element for usize { 
    fn to_element(&self) -> Result<u32, String> { Ok(*self as u32) }
}
