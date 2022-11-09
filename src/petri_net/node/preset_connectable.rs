use std::collections::BTreeSet;

/// Trait to implement adding and removing references to the preset and the postset.
/// The references are of type `Self::RefType`.
pub trait PresetConnectable {
    type RefType: Ord;

    /// Return an immutable reference to the set of nodes with incoming connections to this node.
    fn get_preset(&self) -> &BTreeSet<Self::RefType>;

    /// Return a mutable reference to the set of nodes with incoming connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_preset_mut(&mut self) -> &mut BTreeSet<Self::RefType>;

    /// Add an incoming node, update the preset accordingly.
    fn add_incoming(&mut self, reference: Self::RefType) -> bool {
        self.get_preset_mut().insert(reference)
    }

    /// Remove an incoming node, update the preset accordingly.
    fn remove_incoming(&mut self, reference: &Self::RefType) -> bool {
        self.get_preset_mut().remove(reference)
    }
}
