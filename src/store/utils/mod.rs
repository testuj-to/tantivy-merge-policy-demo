mod index_value;
pub use index_value::{
    index_optional_text,
    index_optional_i64,
    index_optional_facet,
    index_facet,
};

mod merge_policy;
pub use merge_policy::{
    TargetDocsPerSegmentPolicy,
    MergeWheneverPossiblePolicy,
};
