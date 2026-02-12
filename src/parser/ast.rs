pub trait ASTNode: Copy + Clone + std::fmt::Debug + std::hash::Hash + PartialEq + Eq {
    fn follow_sets_contains(&self) -> bool;
    fn sync_points_contains(&self) -> bool {
        self.follow_sets_contains()
    }
}
