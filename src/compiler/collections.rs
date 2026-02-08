pub trait ASTContainer {
    type List<T>: IntoIterator<Item = T>;
    type Boxed<T>;
    type Alocator;

    fn list_new<T, I: IntoIterator<Item = T>>(
        alocator: &Self::Alocator,
        iterator: I,
    ) -> Self::List<T>;
    fn box_new<T>(alocator: Self::Alocator, item: T) -> Self::Boxed<T>;
}
