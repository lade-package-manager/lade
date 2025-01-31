

pub enum RhaiErr<T, E>{
    RErr(E),
    ROk(T),
}


impl<T, E> RhaiErr<T, E>{
    pub fn unwrap(self) -> T{
        match self{
            RhaiErr::RErr(_) => {
                panic!("Called `unwrap`");
            }
            RhaiErr::ROk(t) => t,
        }
    }
}