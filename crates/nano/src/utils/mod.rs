mod walk_declaration_blocks;

pub(crate) use walk_declaration_blocks::{
    ConditionalFrame, DeclarationBlockCollector, DeclarationBlockDiscovery,
    DeclarationBlockEntryId, DeclarationBlockKind, EffectiveKeyId, OpaqueConditionalKind,
    RuleListId, RuleListSegmentId, SelectorFrameKind, SiblingOrdinal, StructuralLocation,
    WalkState, ends_rule_list_segment,
};

#[cfg(test)]
pub(crate) use walk_declaration_blocks::{discover_declaration_blocks, walk_declaration_blocks};
