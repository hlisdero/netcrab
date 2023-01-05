use std::collections::BTreeSet;

/// Trait to implement adding and removing references to the preset and the postset.
/// The references are of type `Self::RefType`.
pub trait PresetConnectable {
    type RefType: Ord;

    /// Returns an immutable reference to the set of nodes with incoming connections to this node.
    fn get_preset(&self) -> &BTreeSet<Self::RefType>;

    /// Returns a mutable reference to the set of nodes with incoming connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_preset_mut(&mut self) -> &mut BTreeSet<Self::RefType>;

    /// Adds an incoming node, update the preset accordingly.
    #[inline]
    fn add_incoming(&mut self, reference: Self::RefType) -> bool {
        self.get_preset_mut().insert(reference)
    }

    /// Removes an incoming node, update the preset accordingly.
    #[inline]
    fn remove_incoming(&mut self, reference: &Self::RefType) -> bool {
        self.get_preset_mut().remove(reference)
    }
}
