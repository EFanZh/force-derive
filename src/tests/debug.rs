use std::fmt::Debug;
use std::marker::PhantomData;

struct NotDebug;

// Unit.

#[derive(force_derive_impl::Debug)]
struct UnitDebug1;

#[derive(force_derive_impl::Debug)]
struct UnitDebug2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::Debug)]
struct TupleDebug1<T>(PhantomData<T>);

#[derive(force_derive_impl::Debug)]
struct TupleDebug2<T>(PhantomData<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::Debug)]
enum EnumDebugEmpty {}

#[derive(force_derive_impl::Debug)]
enum EnumDebugSingle<T> {
    B(PhantomData<T>),
}

#[derive(force_derive_impl::Debug)]
enum EnumDebug1<T> {
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

#[derive(force_derive_impl::Debug)]
enum EnumDebug2<T>
where
    T: Send + ?Sized,
{
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitDebug1: Debug);
static_assertions::assert_impl_all!(UnitDebug2: Debug);
static_assertions::assert_impl_all!(TupleDebug1<NotDebug>: Debug);
static_assertions::assert_impl_all!(TupleDebug2<NotDebug>: Debug);
static_assertions::assert_impl_all!(EnumDebugEmpty: Debug);
static_assertions::assert_impl_all!(EnumDebugSingle<NotDebug>: Debug);
static_assertions::assert_impl_all!(EnumDebug1<NotDebug>: Debug);
static_assertions::assert_impl_all!(EnumDebug2<NotDebug>: Debug);

fn debug<T>(value: &T) -> String
where
    T: Debug,
{
    format!("{value:?}")
}

#[test]
fn test_debug() {
    // Unit.

    assert_eq!(debug(&UnitDebug1), "UnitDebug1");
    assert_eq!(debug(&UnitDebug2), "UnitDebug2");

    // Tuple.

    assert_eq!(
        debug(&TupleDebug1::<NotDebug>(PhantomData)),
        "TupleDebug1(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(
        debug(&TupleDebug2::<NotDebug>(PhantomData)),
        "TupleDebug2(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    // Enum.

    assert_eq!(
        debug(&EnumDebugSingle::<NotDebug>::B(PhantomData)),
        "B(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(debug(&EnumDebug1::<NotDebug>::A), "A");

    assert_eq!(
        debug(&EnumDebug1::<NotDebug>::B(PhantomData)),
        "B(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(
        debug(&EnumDebug1::<NotDebug>::C { bar: PhantomData }),
        "C { bar: PhantomData<force_derive::tests::debug::NotDebug> }",
    );

    assert_eq!(debug(&EnumDebug2::<NotDebug>::A), "A");

    assert_eq!(
        debug(&EnumDebug2::<NotDebug>::B(PhantomData)),
        "B(PhantomData<force_derive::tests::debug::NotDebug>)",
    );

    assert_eq!(
        debug(&EnumDebug2::<NotDebug>::C { bar: PhantomData }),
        "C { bar: PhantomData<force_derive::tests::debug::NotDebug> }",
    );
}
