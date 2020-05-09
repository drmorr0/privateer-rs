use crate::state_machine::State;

// This trick helpfully stolen from StackOverflow
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
//
// Like the selected answer, I have no idea how this works
pub trait BoxClone {
    fn box_clone(&self) -> Box<dyn State>;
}

impl<T> BoxClone for T
where
    T: 'static + State + Clone,
{
    fn box_clone(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.box_clone()
    }
}
