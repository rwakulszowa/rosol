use package::ident::Ident;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dependency<Id: Ident> {
    Choice(Vec<Id>)
}

impl<Id: Ident> Dependency<Id> {
    pub fn ids(&self) -> Vec<Id> {
        match self {
            &Dependency::Choice(ref ids) => ids.clone()
        }
    }
}
