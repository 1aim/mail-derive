impl InspectEmbeddedResources for Au {
    fn inspect_resources(&self, visitor: &mut FnMut(&Embedded)) {
        #[allow(unused_variables)]
        match self {
            &Au(ref f0, ref f1, ) => {
                f0.inspect_resources(visitor);
                f1.inspect_resources(visitor);
            }
        }
    }
    fn inspect_resources_mut(&mut self, visitor: &mut FnMut (&mut Embedded)) {
        #[allow(unused_variables)]
        match self {
            &mut Au(ref mut f0, ref mut f1, ) => {
                f0.inspect_resources_mut(visitor);
                f1.inspect_resources_mut(visitor);
            }
        }
    }
}
