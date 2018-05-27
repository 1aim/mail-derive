enum A {
    VariA,
    VariB(u32, u32, #[mail(inspect_skip)] u8),
    VariC {
        f1: u32,
        #[mail(inspect_with="(afn, afn_mut)")]
        f2: u32
    }
}