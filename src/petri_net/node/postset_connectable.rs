use std::collections::BTreeSet;

/// Trait to implement adding and removing references to the postset.
/// The references are of type `Self::RefType`.
pub trait PostsetConnectable {
    type RefType: Ord;

    /// Return an immutable reference the set of nodes with outgoing connections to this node.
    fn get_postset(&self) -> &BTreeSet<Self::RefType>;

    /// Return a mutable reference the set of nodes with outgoing connections to this node.
    /// We need mutable access to this set to implement adding and removing nodes.
    fn get_postset_mut(&mut self) -> &mut BTreeSet<Self::RefType>;

    /// Add an outgoing node, update the postset accordingly.
    #[inline]
    fn add_outgoing(&mut self, reference: Self::RefType) -> bool {
        self.get_postset_mut().insert(reference)
    }

    /// Remove an outgoing node, update the postset accordingly.
    #[inline]
    fn remove_outgoing(&mut self, reference: &Self::RefType) -> bool {
        self.get_postset_mut().remove(reference)
    }
}
