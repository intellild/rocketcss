use rustc_hash::FxHashSet;
use std::collections::{BTreeSet, VecDeque};

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
pub(super) struct SameSelectorCandidateList(VecDeque<Candidate>);

impl SameSelectorCandidateList {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }

    pub(super) fn push(&mut self, candidate: Candidate) {
        self.0.push_back(candidate);
    }

    pub(super) fn pop(&mut self) -> Option<Candidate> {
        self.0.pop_front()
    }
}

#[derive(Debug, Default)]
pub(super) struct DeclarationOverrideCandidateList(CandidateQueue);

impl DeclarationOverrideCandidateList {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self(CandidateQueue {
            candidates: VecDeque::with_capacity(capacity),
            queued: FxHashSet::default(),
        })
    }

    pub(super) fn push(&mut self, candidate: Candidate) {
        self.0.push(candidate);
    }

    pub(super) fn pop(&mut self) -> Option<Candidate> {
        self.0.pop()
    }
}

#[derive(Debug, Default)]
pub(super) struct PartialMergeCandidateList {
    candidates: BTreeSet<Candidate>,
}

impl PartialMergeCandidateList {
    #[allow(dead_code)]
    pub(super) fn push(&mut self, candidate: Candidate) {
        self.candidates.insert(candidate);
    }

    pub(super) fn pop(&mut self) -> Option<Candidate> {
        self.candidates.pop_first()
    }
}
