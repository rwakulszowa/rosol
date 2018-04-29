use std::cmp::Eq;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::marker::PhantomData;

use package::ident::Ident;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Dependency<Id: Ident> {
    Choice(Vec<Id>)
}

impl<Id: Ident> Dependency<Id> {
    fn ids(&self) -> Vec<Id> {
        match self {
            &Dependency::Choice(ref ids) => ids.clone()
        }
    }
}
