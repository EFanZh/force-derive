use std::fmt::Debug;
use std::marker::PhantomData;

struct NotDebug;

// Struct.

#[derive(force_derive_impl::Debug)]
struct StructDebug0 {}

#[derive(force_derive_impl::Debug)]
struct StructDebug1<T> {
    foo: PhantomData<T>,
}

#[derive(force_derive_impl::Debug)]
struct StructDebug2<T>
where
    u32: Copy,
{
    foo: PhantomData<T>,
    bar: PhantomData<T>,
}

// Tuple.

#[derive(force_derive_impl::Debug)]
struct TupleDebug0();

#[derive(force_derive_impl::Debug)]
struct TupleDebug1<T>(PhantomData<T>);

#[derive(force_derive_impl::Debug)]
struct TupleDebug2<T>(PhantomData<T>, PhantomData<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Debug)]
struct UnitDebug;

// Enum.

#[derive(force_derive_impl::Debug)]
enum EnumDebug0 {}

#[derive(force_derive_impl::Debug)]
enum EnumDebug1<T> {
    Tuple1(PhantomData<T>),
}

#[derive(force_derive_impl::Debug)]
enum EnumDebug<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 { foo: PhantomData<T> },
    Struct2 { foo: PhantomData<T>, bar: PhantomData<T> },
    Tuple0(),
    Tuple1(PhantomData<T>),
    Tuple2(PhantomData<T>, PhantomData<T>),
    Unit,
}

// Special identifiers.

#[allow(dead_code)]
#[derive(force_derive_impl::Debug)]
struct SpecialIdentifierStructDebug {
    f: u32,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(force_derive_impl::Debug)]
enum SpecialIdentifierEnumDebug {
    Struct { f: u32 },
}

// Tests.

static_assertions::assert_impl_all!(StructDebug0: Debug);
static_assertions::assert_impl_all!(StructDebug1<NotDebug>: Debug);
static_assertions::assert_impl_all!(StructDebug2<NotDebug>: Debug);
static_assertions::assert_impl_all!(TupleDebug0: Debug);
static_assertions::assert_impl_all!(TupleDebug1<NotDebug>: Debug);
static_assertions::assert_impl_all!(TupleDebug2<NotDebug>: Debug);
static_assertions::assert_impl_all!(UnitDebug: Debug);
static_assertions::assert_impl_all!(EnumDebug0: Debug);
static_assertions::assert_impl_all!(EnumDebug1<NotDebug>: Debug);
static_assertions::assert_impl_all!(EnumDebug<NotDebug>: Debug);

fn debug<T>(value: &T) -> String
where
    T: Debug,
{
    format!("{value:?}")
}

#[test]
fn test_debug() {
    // Struct.

    assert_eq!(debug(&StructDebug0 {}), "StructDebug0");

    assert_eq!(
        debug(&StructDebug1::<NotDebug> { foo: PhantomData }),
        "StructDebug1 { foo: PhantomData<force_derive::tests::debug::NotDebug> }",
    );

    assert_eq!(
        debug(&StructDebug2::<NotDebug> {
            foo: PhantomData,
            bar: PhantomData,
        }),
        "StructDebug2 { foo: PhantomData<force_derive::tests::debug::NotDebug>, bar: PhantomData<force_derive::tests::debug::NotDebug> }",
    );

    // Tuple.

    assert_eq!(debug(&TupleDebug0()), "TupleDebug0");

    assert_eq!(
        debug(&TupleDebug1::<NotDebug>(PhantomData)),
        "TupleDebug1(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(
        debug(&TupleDebug2::<NotDebug>(PhantomData, PhantomData)),
        "TupleDebug2(PhantomData<force_derive::tests::debug::NotDebug>, PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    // Unit.

    assert_eq!(debug(&UnitDebug), "UnitDebug");

    // Enum.

    assert_eq!(
        debug(&EnumDebug1::<NotDebug>::Tuple1(PhantomData)),
        "Tuple1(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(debug(&EnumDebug::<NotDebug>::Struct0 {}), "Struct0");

    assert_eq!(
        debug(&EnumDebug::<NotDebug>::Struct1 { foo: PhantomData }),
        "Struct1 { foo: PhantomData<force_derive::tests::debug::NotDebug> }",
    );

    assert_eq!(
        debug(&EnumDebug::<NotDebug>::Struct2 {
            foo: PhantomData,
            bar: PhantomData,
        }),
        "Struct2 { foo: PhantomData<force_derive::tests::debug::NotDebug>, bar: PhantomData<force_derive::tests::debug::NotDebug> }",
    );

    assert_eq!(debug(&EnumDebug::<NotDebug>::Tuple0()), "Tuple0");

    assert_eq!(
        debug(&EnumDebug::<NotDebug>::Tuple1(PhantomData)),
        "Tuple1(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(
        debug(&EnumDebug::<NotDebug>::Tuple2(PhantomData, PhantomData)),
        "Tuple2(PhantomData<force_derive::tests::debug::NotDebug>, PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(debug(&EnumDebug::<NotDebug>::Unit), "Unit");

    // Special identifiers.

    assert_eq!(
        debug(&SpecialIdentifierStructDebug { f: 2 }),
        "SpecialIdentifierStructDebug { f: 2 }",
    );

    assert_eq!(debug(&SpecialIdentifierEnumDebug::Struct { f: 2 }), "Struct { f: 2 }");
}
