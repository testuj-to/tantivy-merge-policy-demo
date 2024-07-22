use tantivy::{
    index::{
        SegmentId,
        SegmentMeta,
    },
    merge_policy::{
        MergePolicy,
        MergeCandidate,
    },
};

// TargetDocsPerSegmentPolicy

#[derive(Debug, Clone)]
pub struct TargetDocsPerSegmentPolicy {
    run_id: String,
    target_docs_per_segment: u32,
}

impl TargetDocsPerSegmentPolicy {
    pub fn new(run_id: String, target_docs_per_segment: u32) -> Self {
        TargetDocsPerSegmentPolicy{
            run_id,
            target_docs_per_segment,
        }
    }

    pub fn as_box(self) -> Box<Self> {
        return Box::new(self);
    }
}

impl MergePolicy for TargetDocsPerSegmentPolicy {
    fn compute_merge_candidates(&self, segment_metas: &[SegmentMeta]) -> Vec<MergeCandidate> {
        println!("{{\"run\":\"{}\",\"count\":\"{}\"}}", self.run_id, segment_metas.len());

        let mut merge_candidates: Vec<(u32, Vec<SegmentId>)> = Vec::new();

        'merge_segment_loop: for segment in segment_metas {
            let segment_id = segment.id();
            let num_docs = segment.num_docs();

            for group in merge_candidates.iter_mut() {
                if group.0 + num_docs < self.target_docs_per_segment {
                    group.1.push(segment_id);
                    continue 'merge_segment_loop;
                }
            }

            if merge_candidates.len() < 1 {
                merge_candidates.push((num_docs, vec![segment_id]));
            }
        }

        merge_candidates
            .into_iter()
            .map(|group| MergeCandidate(group.1))
            .collect::<Vec<MergeCandidate>>()
    }
}

// MergeWheneverPossiblePolicy

#[derive(Debug, Clone)]
pub struct MergeWheneverPossiblePolicy {
    run_id: String,
}

impl MergeWheneverPossiblePolicy {
    pub fn new(run_id: String) -> Self {
        MergeWheneverPossiblePolicy{ run_id }
    }

    pub fn as_box(self) -> Box<Self> {
        return Box::new(self);
    }
}

impl MergePolicy for MergeWheneverPossiblePolicy {
    fn compute_merge_candidates(&self, segment_metas: &[SegmentMeta]) -> Vec<MergeCandidate> {
        println!("{{\"run\":\"{}\",\"count\":\"{}\"}}", self.run_id, segment_metas.len());

        let segment_ids = segment_metas
            .iter()
            .map(|segment_meta| segment_meta.id())
            .collect::<Vec<SegmentId>>();

        if segment_ids.len() > 1 {
            return vec![MergeCandidate(segment_ids)];
        }

        vec![]
    }
}
