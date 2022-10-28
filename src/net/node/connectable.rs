use std::collections::HashSet;
use std::hash::Hash;

/// Trait to implement adding and removing references to the preset and the postset.
/// The references are of type `Self::RefType`.
pub trait Connectable {
    type RefType: Eq + Hash;

    /// Return a mutable reference to the set of nodes with incoming connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_preset(&mut self) -> &mut HashSet<Self::RefType>;

    /// Return a mutable reference the set of nodes with outgoing connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_postset(&mut self) -> &mut HashSet<Self::RefType>;

    /// Add an incoming node, update the preset accordingly.
    fn add_incoming(&mut self, reference: Self::RefType) -> bool {
        self.get_preset().insert(reference)
    }

    /// Remove an incoming node, update the preset accordingly.
    fn remove_incoming(&mut self, reference: &Self::RefType) -> bool {
        self.get_preset().remove(reference)
    }

    /// Add an outgoing node, update the postset accordingly.
    fn add_outgoing(&mut self, reference: Self::RefType) -> bool {
        self.get_postset().insert(reference)
    }

    /// Remove an outgoing node, update the postset accordingly.
    fn remove_outgoing(&mut self, reference: &Self::RefType) -> bool {
        self.get_postset().remove(reference)
    }
}
