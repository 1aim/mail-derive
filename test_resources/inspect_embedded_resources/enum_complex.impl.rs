impl InspectEmbeddedResources for A {
    fn inspect_resources(&self, visitor: &mut FnMut(&Embedded)) {
        #[allow(unused_variables)]
        match self {
            &A::VariB(ref f0, ref f1, ref f2, ) => {
                f0.inspect_resources(visitor);
                f1.inspect_resources(visitor);
            },
            &A::VariC { ref f1, ref f2, } => {
                f1.inspect_resources(visitor);
                afn(f2, visitor);
            },
        }
    }
    fn inspect_resources_mut(&mut self, visitor: &mut FnMut(&mut Embedded)) {
        #[allow(unused_variables)]
        match self {
            &mut A::VariB(ref mut f0, ref mut f1, ref mut f2, ) => {
                f0.inspect_resources_mut(visitor);
                f1.inspect_resources_mut(visitor);
            },
            &mut A::VariC { ref mut f1, ref mut f2, } => {
                f1.inspect_resources_mut(visitor);
                afn_mut(f2, visitor);
            },
        }
    }
}