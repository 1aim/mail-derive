# mail-derive

**This crate provides a custom derives for the mail crate.**

---

Currently it only contains the `InspectEmbeddedResource`
derive which allows deriving the `InspectEmbeddedResource`
trait from the `mail-template` (sub-)crate.

## InspectEmbeddedResource

Implements `InspectEmbeddedResource` by forwarding calls to it's
methods to all fields.

**Belongs to**: `mail-template`

**Applicable to**: struct, tuple struct, enum, enum with named fields

**Attribute Scope**: `mail` (e.g. `#[mail(inspect_skip)]`)

**Field Attributes**:

- `inspect_skip`, don't forward calls to the annotated _field_
- `inspect_with = "(no_mut_fn, mut_fn)"`, specifies two function
  which are called with a reference to the field (1st param) and
  the visitor (2nd param) instead of calling inspect_resource(_mut)
  on the field. Note that the functions in the tuple are interpreted
  as paths so e.g. `::some_thing::fn1` would be potentially valid.

(Note that field attributes apply to any kind of field wether it's
named or unnamed appears in a struct or enum.)

**Type Attributes**: None

**Enum Variant Attributes**: None