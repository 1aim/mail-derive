impl InspectEmbeddedResources for A {
    fn inspect_resources(&self, visitor: &mut FnMut(&Embedded)) {
        #[allow(unused_variables)]
        match self {
            &A { ref f1, ref fa, ref fe, ref f2, } => {
                fa.inspect_resources(visitor);
                la::special(fe, visitor);
                f2.inspect_resources(visitor);
            }
        }
    }
    fn inspect_resources_mut(&mut self, visitor: &mut FnMut (&mut Embedded)) {
        #[allow(unused_variables)]
        match self {
            &mut A { ref mut f1, ref mut fa, ref mut fe, ref mut f2, } => {
                fa.inspect_resources_mut(visitor);
                la::special_mut(fe, visitor);
                f2.inspect_resources_mut(visitor);
            }
        }
    }
}