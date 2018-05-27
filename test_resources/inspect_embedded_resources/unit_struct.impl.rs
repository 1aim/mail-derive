impl InspectEmbeddedResources for A {
    fn inspect_resources(&self, visitor: &mut impl FnMut(&Embedded)) {
    }
    fn inspect_resources_mut(&mut self, visitor: &mut impl FnMut (&mut Embedded)) {
    }
}