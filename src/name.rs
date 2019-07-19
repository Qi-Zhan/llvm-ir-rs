/// Many LLVM objects have a `Name`, which is either a string name, or just a
/// sequential numbering (e.g. `%3`).
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Name {
    /// has a string name
    Name(String),
    /// doesn't have a string name and was given this sequential number
    Number(usize),
}

impl Name {
    pub(crate) fn name_or_num(s: String, ctr: &mut usize) -> Self {
        if s != "" {
            Name::Name(s)
        } else {
            let rval = Name::Number(*ctr);
            *ctr += 1;
            rval
        }
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Name::Name(s)
    }
}

impl From<usize> for Name {
    fn from(u: usize) -> Self {
        Name::Number(u)
    }
}
