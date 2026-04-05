use crate::acl::pattern::{Action, ActionWithOrigin};
#[cfg(test)]
pub mod simple_flow;

/// Iterator over ACL actions
pub trait Flow {
    fn next(&mut self) -> Option<Action>;
    fn peek(&self) -> Option<Action>;

    fn next_with_origin(&mut self) -> Option<ActionWithOrigin>;
    fn peek_with_origin(&self) -> Option<ActionWithOrigin>;
}
