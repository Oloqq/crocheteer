use crate::acl::pattern::Action;
#[cfg(test)]
pub mod simple_flow;

/// Iterator over ACL actions
pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;
}
