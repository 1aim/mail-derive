# mail-derive

**This crate provides a custom derives for the mail crate.**

---
This crate provides a custom derives for the mail crate.

Currently it only contains the `InspectEmbeddedResource`
derive which allows deriving the `InspectEmbeddedResource`
trait from the `mail-template` (sub-)crate.

**Note this crate will be re-exported through the mail
 facade in the same way as the other mail-* crates are.**

# InspectEmbeddedResource

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

**Enum Variant Attributes**:

- `inspect_skip`, skip over all fields in this enum variant

## Example

```rust
extern crate mail_template;
#[macro_use]
extern crate mail_derive;

use mail_template::{Embedded, InspectEmbeddedResources};


// Just take this as any kind of type.
type SomeRandomType = u32;

// Let's assume this is imported from another crate.
struct TypeNotImplTrait(SomeRandomType);

#[derive(InspectEmbeddedResources)]
enum A {
  VariA,
  VariB(SomeRandomType, #[mail(inspect_skip)] TypeNotImplTrait),
  VariC {
    f1: SomeRandomType,
    #[mail(inspect_with="(inspect, inspect_mut)")]
    f2: TypeNotImplTrait
  }
}

fn inspect(me: &TypeNotImplTrait, visitor: &mut FnMut(&Embedded)) {
  me.0.inspect_resources(visitor)
}
fn inspect_mut(me: &mut TypeNotImplTrait, visitor: &mut FnMut(&mut Embedded)) {
  me.0.inspect_resources_mut(visitor)
}

# fn main() {}
```