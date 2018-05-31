impl InspectEmbeddedResources for A {
    fn inspect_resources(&self, visitor: &mut FnMut(&Embedded)) {
    }
    fn inspect_resources_mut(&mut self, visitor: &mut FnMut (&mut Embedded)) {
    }
}