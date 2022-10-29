use std::collections::HashSet;
use std::hash::Hash;

/// Trait to implement adding and removing references to the preset and the postset.
/// The references are of type `Self::RefType`.
pub trait ConnectableNode {
    type RefType: Eq + Hash;

    /// Return an immutable reference to the set of nodes with incoming connections to this node.
    fn get_preset(&self) -> &HashSet<Self::RefType>;

    /// Return an immutable reference the set of nodes with outgoing connections to this node.
    fn get_postset(&self) -> &HashSet<Self::RefType>;

    /// Return a mutable reference to the set of nodes with incoming connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_preset_mut(&mut self) -> &mut HashSet<Self::RefType>;

    /// Return a mutable reference the set of nodes with outgoing connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_postset_mut(&mut self) -> &mut HashSet<Self::RefType>;

    /// Add an incoming node, update the preset accordingly.
    fn add_incoming(&mut self, reference: Self::RefType) -> bool {
        self.get_preset_mut().insert(reference)
    }

    /// Remove an incoming node, update the preset accordingly.
    fn remove_incoming(&mut self, reference: &Self::RefType) -> bool {
        self.get_preset_mut().remove(reference)
    }

    /// Add an outgoing node, update the postset accordingly.
    fn add_outgoing(&mut self, reference: Self::RefType) -> bool {
        self.get_postset_mut().insert(reference)
    }

    /// Remove an outgoing node, update the postset accordingly.
    fn remove_outgoing(&mut self, reference: &Self::RefType) -> bool {
        self.get_postset_mut().remove(reference)
    }
}
