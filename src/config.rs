use serde::{Deserialize, Serialize};

use crate::commit::{cmt_type::CommitType, strategy::CaseStrategy};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config<'a: 'b, 'b> {
    #[serde(borrow)]
    _phantom: std::marker::PhantomData<&'a ()>,
    pub commit_types: Vec<CommitType<'b>>,
    pub strategy: CaseStrategy,
}

impl<'a: 'b, 'b> Config<'a, 'b> {
    pub fn new(
        commit_types: Vec<CommitType<'b>>,
        strategy: CaseStrategy,
    ) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            commit_types,
            strategy,
        }
    }
}
