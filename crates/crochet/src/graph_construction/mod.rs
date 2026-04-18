pub mod errors;
mod hook;

pub use errors::{Error as HookError, ErrorCode};
pub use hook::HookParams;

use crate::{acl::Flow, data::InitialGraph};

pub(crate) fn parse(mut flow: impl Flow, params: HookParams) -> Result<InitialGraph, HookError> {
    if flow.peek().is_none() {
        return Err(HookError {
            code: ErrorCode::Empty,
            origin: None,
        });
    }
    let mut hook = hook::Hook::new(params);
    let mut i: u32 = 0;
    while let Some(action_with_origin) = flow.next_with_origin() {
        let action = action_with_origin.action;
        let origin = action_with_origin.origin;
        log::trace!("Performing [{i}] {action:?}. Origin: {origin:?}");
        i += 1;
        hook = match hook.perform(&action, origin) {
            Ok(hook) => hook,
            Err(err) => return Err(HookError { code: err, origin }),
        };
    }

    let result = hook.finish();
    Ok(result)
}
