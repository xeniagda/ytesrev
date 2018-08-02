
/// Represents an object that has to load certain resources
pub trait Loadable {
    fn register(&mut self) {}
    fn load(&mut self);
}

default impl <T> Loadable for T {
    fn register(&mut self) {}
    fn load(&mut self) {}
}

#[macro_export]
macro_rules! impl_loadable {
    ($obj:ty, $( $content:ident ),+) => {
        impl Loadable for $obj {
            fn register(&mut self) { $( self.$content.register() );* }
            fn load(&mut self) { $( self.$content.load() );* }
        }
    }
}
