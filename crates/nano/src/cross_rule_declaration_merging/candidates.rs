use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Candidate(pub(super) u32, pub(super) u32);

#[derive(Debug, Default)]
struct CandidateQueue {
    candidates: VecDeque<Candidate>,
    queued: FxHashSet<Candidate>,
}

impl CandidateQueue {
    fn push(&mut self, candidate: Candidate) {
        if self.queued.insert(candidate) {
            self.candidates.push_back(candidate);
        }
    }

    fn pop(&mut self) -> Option<Candidate> {
        let candidate = self.candidates.pop_front()?;
        self.queued.remove(&candidate);
        Some(candidate)
    }
}

#[derive(Debug, Default)]
pub(super) struct SameSelectorCandidateList(CandidateQueue);

impl SameSelectorCandidateList {
    pub(super) fn push(&mut self, candidate: Candidate) {
        self.0.push(candidate);
    }

    pub(super) fn pop(&mut self) -> Option<Candidate> {
        self.0.pop()
    }
}
