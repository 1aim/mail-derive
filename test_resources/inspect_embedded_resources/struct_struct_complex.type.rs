struct A {
    #[mail(inspect_skip)]
    f1: u32,
    fa: f64,
    #[mail(inspect_with="(la::special, la::special_mut)")]
    fe: i32,
    f2: Resource
}