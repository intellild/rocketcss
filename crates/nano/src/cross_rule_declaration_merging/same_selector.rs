use super::live_sibling_graph::LiveSiblingGraph;

impl LiveSiblingGraph {
    pub(super) fn stabilize_same_selector_candidates(&mut self) {
        while let Some(candidate) = self.pop_same_selector_candidate() {
            let Some((left, right)) = self.candidate_is_live_edge(candidate) else {
                continue;
            };
            self.concatenate_and_retire_left(left, right);
        }
    }
}
