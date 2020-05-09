use crate::components::Component;

// This trick helpfully stolen from StackOverflow
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
//
// Like the selected answer, I have no idea how this works
pub trait BoxClone {
    fn box_clone(&self) -> Box<dyn Component>;
}

impl<T> BoxClone for T
where
    T: 'static + Component + Clone,
{
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.box_clone()
    }
}
