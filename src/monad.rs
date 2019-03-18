impl<T> Option<T> {
    pub fn liftA2<A, B, C> (F: Fn(A, B) -> C, option1: Option<A>, 
        option2: Option<B>) -> Option<C> 
    {
        match (option1, option2) {
            (Some(A), Some(B)) => Some(F (A, B)),
            _ => None,
        }
    }
}