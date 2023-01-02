use std::marker::PhantomData;

struct NotPartialOrd;

// Struct.

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct StructPartialOrd0 {}

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct StructPartialOrd1<T> {
    foo: PhantomData<T>,
}

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct StructPartialOrd2<T>
where
    u32: Copy,
{
    foo: PhantomData<T>,
    bar: u32,
}

// Tuple.

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct TuplePartialOrd0();

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct TuplePartialOrd1<T>(PhantomData<T>);

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct TuplePartialOrd2<T>(PhantomData<T>, u32)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct UnitPartialOrd;

// Enum.

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
enum EnumPartialOrd0 {}

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
enum EnumPartialOrd1<T> {
    Tuple1(PhantomData<T>),
}

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
enum EnumPartialOrd<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 { foo: PhantomData<T> },
    Struct2 { foo: PhantomData<T>, bar: u32 },
    Tuple0(),
    Tuple1(PhantomData<T>),
    Tuple2(PhantomData<T>, u32),
    Unit,
}

// Tests.

static_assertions::assert_impl_all!(StructPartialOrd0: PartialOrd);
static_assertions::assert_impl_all!(StructPartialOrd1<NotPartialOrd>: PartialOrd);
static_assertions::assert_impl_all!(StructPartialOrd2<NotPartialOrd>: PartialOrd);
static_assertions::assert_impl_all!(TuplePartialOrd0: PartialOrd);
static_assertions::assert_impl_all!(TuplePartialOrd1<NotPartialOrd>: PartialOrd);
static_assertions::assert_impl_all!(TuplePartialOrd2<NotPartialOrd>: PartialOrd);
static_assertions::assert_impl_all!(UnitPartialOrd: PartialOrd);
static_assertions::assert_impl_all!(EnumPartialOrd0: PartialOrd);
static_assertions::assert_impl_all!(EnumPartialOrd1<NotPartialOrd>: PartialOrd);
static_assertions::assert_impl_all!(EnumPartialOrd<NotPartialOrd>: PartialOrd);

fn partial_ord<T>(lhs: &T, rhs: &T) -> (bool, bool)
where
    T: PartialOrd,
{
    (PartialOrd::eq(lhs, rhs), PartialOrd::ne(lhs, rhs))
}

// Special identifiers.

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
struct SpecialIdentifierStructPartialOrd {
    other: u32,
}

#[derive(force_derive_impl::PartialEq, force_derive_impl::PartialOrd)]
enum SpecialIdentifierEnumPartialOrd {
    Struct { other: u32 },
}

#[test]
fn test_partial_ord_struct() {
    assert_eq!(partial_ord(&StructPartialOrd0 {}, &StructPartialOrd0 {}), (true, false));

    assert_eq!(
        partial_ord(
            &StructPartialOrd1::<NotPartialOrd> { foo: PhantomData },
            &StructPartialOrd1::<NotPartialOrd> { foo: PhantomData },
        ),
        (true, false),
    );

    assert_eq!(
        partial_ord(
            &StructPartialOrd2::<NotPartialOrd> {
                foo: PhantomData,
                bar: 2,
            },
            &StructPartialOrd2::<NotPartialOrd> {
                foo: PhantomData,
                bar: 2,
            },
        ),
        (true, false),
    );

    assert_eq!(
        partial_ord(
            &StructPartialOrd2::<NotPartialOrd> {
                foo: PhantomData,
                bar: 2,
            },
            &StructPartialOrd2::<NotPartialOrd> {
                foo: PhantomData,
                bar: 3,
            },
        ),
        (false, true),
    );
}

#[test]
fn test_partial_ord_tuple() {
    assert_eq!(partial_ord(&TuplePartialOrd0 {}, &TuplePartialOrd0 {}), (true, false));

    assert_eq!(
        partial_ord(
            &TuplePartialOrd1::<NotPartialOrd>(PhantomData),
            &TuplePartialOrd1::<NotPartialOrd>(PhantomData),
        ),
        (true, false),
    );

    assert_eq!(
        partial_ord(
            &TuplePartialOrd2::<NotPartialOrd>(PhantomData, 2),
            &TuplePartialOrd2::<NotPartialOrd>(PhantomData, 2),
        ),
        (true, false),
    );

    assert_eq!(
        partial_ord(
            &TuplePartialOrd2::<NotPartialOrd>(PhantomData, 2),
            &TuplePartialOrd2::<NotPartialOrd>(PhantomData, 3),
        ),
        (false, true),
    );
}

#[test]
fn test_partial_ord_unit() {
    assert_eq!(partial_ord(&UnitPartialOrd, &UnitPartialOrd), (true, false));
}

#[test]
fn test_partial_ord_enum_1() {
    assert_eq!(
        partial_ord(
            &EnumPartialOrd1::<NotPartialOrd>::Tuple1(PhantomData),
            &EnumPartialOrd1::<NotPartialOrd>::Tuple1(PhantomData),
        ),
        (true, false)
    );
}

#[test]
fn test_partial_ord_enum() {
    let struct_0 = || EnumPartialOrd::<NotPartialOrd>::Struct0 {};
    let struct_1 = || EnumPartialOrd::<NotPartialOrd>::Struct1 { foo: PhantomData };

    let struct_2_2 = || EnumPartialOrd::<NotPartialOrd>::Struct2 {
        foo: PhantomData,
        bar: 2,
    };

    let struct_2_3 = || EnumPartialOrd::<NotPartialOrd>::Struct2 {
        foo: PhantomData,
        bar: 3,
    };

    let tuple_0 = EnumPartialOrd::<NotPartialOrd>::Tuple0;
    let tuple_1 = || EnumPartialOrd::<NotPartialOrd>::Tuple1(PhantomData);
    let tuple_2_2 = || EnumPartialOrd::<NotPartialOrd>::Tuple2(PhantomData, 2);
    let tuple_2_3 = || EnumPartialOrd::<NotPartialOrd>::Tuple2(PhantomData, 3);
    let unit = || EnumPartialOrd::<NotPartialOrd>::Unit;

    let all_values = [
        struct_0, struct_1, struct_2_2, struct_2_3, tuple_0, tuple_1, tuple_2_2, tuple_2_3, unit,
    ];

    for (i, lhs) in all_values.iter().enumerate() {
        for (j, rhs) in all_values.iter().enumerate() {
            assert_eq!(partial_ord(&lhs(), &rhs()), (i == j, i != j));
        }
    }
}

#[test]
fn test_partial_ord_special_identifiers() {
    let struct_2 = || SpecialIdentifierStructPartialOrd { other: 2 };
    let struct_3 = || SpecialIdentifierStructPartialOrd { other: 3 };

    assert_eq!(partial_ord(&struct_2(), &struct_2()), (true, false));
    assert_eq!(partial_ord(&struct_2(), &struct_3()), (false, true));

    let enum_2 = || SpecialIdentifierEnumPartialOrd::Struct { other: 2 };
    let enum_3 = || SpecialIdentifierEnumPartialOrd::Struct { other: 3 };

    assert_eq!(partial_ord(&enum_2(), &enum_2()), (true, false));
    assert_eq!(partial_ord(&enum_2(), &enum_3()), (false, true));
}
