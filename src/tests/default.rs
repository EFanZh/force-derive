#![allow(dead_code)]

use std::marker::PhantomData;

struct NotDefault;

// Struct.

#[derive(force_derive_impl::Default)]
struct StructDefault0 {}

#[derive(force_derive_impl::Default)]
struct StructDefault1<T> {
    foo: Vec<T>,
}

#[derive(force_derive_impl::Default)]
struct StructDefault2<T>
where
    u32: Copy,
{
    foo: Vec<T>,
    bar: PhantomData<T>,
}

// Tuple.

#[derive(force_derive_impl::Default)]
struct TupleDefault0();

#[derive(force_derive_impl::Default)]
struct TupleDefault1<T>(Vec<T>);

#[derive(force_derive_impl::Default)]
struct TupleDefault2<T>(Vec<T>, PhantomData<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Default)]
struct UnitDefault;

// Enum.

#[derive(force_derive_impl::Default)]
enum EnumDefault1<T> {
    #[default]
    Tuple1(Vec<T>),
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultStruct0<T>
where
    u32: Copy,
{
    #[default]
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultStruct1<T>
where
    u32: Copy,
{
    Struct0 {},
    #[default]
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultStruct2<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    #[default]
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultTuple0<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    #[default]
    Tuple0(),
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultTuple1<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    #[default]
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultTuple2<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(Vec<T>),
    #[default]
    Tuple2(Vec<T>, PhantomData<T>),
    Unit,
}

#[derive(force_derive_impl::Default)]
enum EnumDefaultUnit<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: Vec<T>,
    },
    Struct2 {
        foo: Vec<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(Vec<T>),
    Tuple2(Vec<T>, PhantomData<T>),
    #[default]
    Unit,
}

// Tests.

static_assertions::assert_impl_all!(StructDefault0: Default);
static_assertions::assert_impl_all!(StructDefault1<NotDefault>: Default);
static_assertions::assert_impl_all!(StructDefault2<NotDefault>: Default);
static_assertions::assert_impl_all!(TupleDefault0: Default);
static_assertions::assert_impl_all!(TupleDefault1<NotDefault>: Default);
static_assertions::assert_impl_all!(TupleDefault2<NotDefault>: Default);
static_assertions::assert_impl_all!(UnitDefault: Default);
static_assertions::assert_impl_all!(EnumDefault1<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultStruct0::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultStruct1::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultStruct2::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultTuple0::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultTuple1::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultTuple2::<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefaultUnit::<NotDefault>: Default);

#[test]
fn test_default() {
    // Struct.

    assert!(matches!(StructDefault0::default(), StructDefault0 {}));

    assert!(matches!(
        StructDefault1::<NotDefault>::default(),
        StructDefault1::<NotDefault> { foo } if foo.is_empty(),
    ));

    assert!(matches!(
        StructDefault2::<NotDefault>::default(),
        StructDefault2::<NotDefault> { foo, bar: PhantomData } if foo.is_empty(),
    ));

    // Tuple.

    assert!(matches!(TupleDefault0::default(), TupleDefault0()));

    assert!(matches!(
        TupleDefault1::<NotDefault>::default(),
        TupleDefault1::<NotDefault>(x) if x.is_empty(),
    ));

    assert!(matches!(
        TupleDefault2::<NotDefault>::default(),
        TupleDefault2::<NotDefault>(x, PhantomData) if x.is_empty(),
    ));

    // Unit.

    assert!(matches!(UnitDefault::default(), UnitDefault));

    // Enum.

    assert!(matches!(
        EnumDefault1::<NotDefault>::default(),
        EnumDefault1::<NotDefault>::Tuple1(x) if x.is_empty(),
    ));

    assert!(matches!(
        EnumDefaultStruct0::<NotDefault>::default(),
        EnumDefaultStruct0::<NotDefault>::Struct0 {},
    ));

    assert!(matches!(
        EnumDefaultStruct1::<NotDefault>::default(),
        EnumDefaultStruct1::<NotDefault>::Struct1 { foo } if foo.is_empty(),
    ));

    assert!(matches!(
        EnumDefaultStruct2::<NotDefault>::default(),
        EnumDefaultStruct2::<NotDefault>::Struct2 { foo, bar: PhantomData } if foo.is_empty(),
    ));

    assert!(matches!(
        EnumDefaultTuple0::<NotDefault>::default(),
        EnumDefaultTuple0::<NotDefault>::Tuple0(),
    ));

    assert!(matches!(
        EnumDefaultTuple1::<NotDefault>::default(),
        EnumDefaultTuple1::<NotDefault>::Tuple1(foo) if foo.is_empty(),
    ));

    assert!(matches!(
        EnumDefaultTuple2::<NotDefault>::default(),
        EnumDefaultTuple2::<NotDefault>::Tuple2(foo, PhantomData) if foo.is_empty(),
    ));

    assert!(matches!(
        EnumDefaultUnit::<NotDefault>::default(),
        EnumDefaultUnit::<NotDefault>::Unit,
    ));
}
