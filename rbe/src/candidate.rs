use std::{
    fmt::{Display, Write},
    ops::Deref,
};

use crate::{Component, Key, MatchCond, Ref, Value};

// TODO: We are not using the struct yet
#[derive(Debug, Clone)]
pub struct Candidate<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    values: Vec<(K, V, Component, MatchCond<K, V, R>)>,
}

impl<K, V, R> Candidate<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub(crate) fn new(values: Vec<(K, V, Component, MatchCond<K, V, R>)>) -> Candidate<K, V, R> {
        Candidate { values }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(K, V, Component, MatchCond<K, V, R>)> + '_ {
        self.values.iter()
    }
}

impl<K, V, R> Display for Candidate<K, V, R>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Candidate")?;
        for (key, value, component, cond) in self.values.iter() {
            write!(f, "Candidate value: {key} {value} {component} {cond}")?;
        }
        Ok(())
    }
}

/*impl <K,V,R> IntoIterator for Candidate<K,V,R>
where
  K: Key + Display,
  V: Value + Display,
  R: Ref + Display, {
    type Item = (K, V, Component, MatchCond<K, V, R>);

    type IntoIter = ;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}*/

impl<K, V, R> Deref for Candidate<K, V, R>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
{
    type Target = Vec<(K, V, Component, MatchCond<K, V, R>)>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

/*impl <K,V,R> Iterator for Candidate<K,V,R>
where
  K: Key + Display,
  V: Value + Display,
  R: Ref + Display, {
    type Item = (K, V, Component, MatchCond<K, V, R>);

    fn next(&mut self) -> Option<Self::Item> {
        self.values.iter().map(|v| v.clone())
    }
}*/
