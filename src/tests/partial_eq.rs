use std::marker::PhantomData;

struct NotPartialEq;

// Struct.

#[derive(force_derive_impl::PartialEq)]
struct StructPartialEq0 {}

#[derive(force_derive_impl::PartialEq)]
struct StructPartialEq1<T> {
    foo: PhantomData<T>,
}

#[derive(force_derive_impl::PartialEq)]
struct StructPartialEq2<T>
where
    u32: Copy,
{
    foo: PhantomData<T>,
    bar: u32,
}

// Tuple.

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq0();

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq1<T>(PhantomData<T>);

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq2<T>(PhantomData<T>, u32)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::PartialEq)]
struct UnitPartialEq;

// Enum.

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEq0 {}

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEq1<T> {
    Tuple1(PhantomData<T>),
}

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEq<T>
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

static_assertions::assert_impl_all!(StructPartialEq0: PartialEq);
static_assertions::assert_impl_all!(StructPartialEq1<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(StructPartialEq2<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(TuplePartialEq0: PartialEq);
static_assertions::assert_impl_all!(TuplePartialEq1<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(TuplePartialEq2<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(UnitPartialEq: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEq0: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEq1<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEq<NotPartialEq>: PartialEq);

fn partial_eq<T>(lhs: &T, rhs: &T) -> (bool, bool)
where
    T: PartialEq,
{
    (PartialEq::eq(lhs, rhs), PartialEq::ne(lhs, rhs))
}

#[test]
fn test_partial_eq_struct() {
    assert_eq!(
        partial_eq(&StructPartialEq0 {}, &StructPartialEq0 {}),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &StructPartialEq1::<NotPartialEq> { foo: PhantomData },
            &StructPartialEq1::<NotPartialEq> { foo: PhantomData },
        ),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &StructPartialEq2::<NotPartialEq> {
                foo: PhantomData,
                bar: 2,
            },
            &StructPartialEq2::<NotPartialEq> {
                foo: PhantomData,
                bar: 2,
            },
        ),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &StructPartialEq2::<NotPartialEq> {
                foo: PhantomData,
                bar: 2,
            },
            &StructPartialEq2::<NotPartialEq> {
                foo: PhantomData,
                bar: 3,
            },
        ),
        (false, true),
    );
}

#[test]
fn test_partial_eq_tuple() {
    assert_eq!(
        partial_eq(&TuplePartialEq0 {}, &TuplePartialEq0 {}),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &TuplePartialEq1::<NotPartialEq>(PhantomData),
            &TuplePartialEq1::<NotPartialEq>(PhantomData),
        ),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &TuplePartialEq2::<NotPartialEq>(PhantomData, 2),
            &TuplePartialEq2::<NotPartialEq>(PhantomData, 2),
        ),
        (true, false),
    );

    assert_eq!(
        partial_eq(
            &TuplePartialEq2::<NotPartialEq>(PhantomData, 2),
            &TuplePartialEq2::<NotPartialEq>(PhantomData, 3),
        ),
        (false, true),
    );
}

#[test]
fn test_partial_eq_unit() {
    assert_eq!(partial_eq(&UnitPartialEq, &UnitPartialEq), (true, false));
}

#[test]
fn test_partial_eq_enum_1() {
    assert_eq!(
        partial_eq(
            &EnumPartialEq1::<NotPartialEq>::Tuple1(PhantomData),
            &EnumPartialEq1::<NotPartialEq>::Tuple1(PhantomData),
        ),
        (true, false)
    );
}

#[test]
fn test_partial_eq_enum() {
    let struct_0 = || EnumPartialEq::<NotPartialEq>::Struct0 {};
    let struct_1 = || EnumPartialEq::<NotPartialEq>::Struct1 { foo: PhantomData };

    let struct_2_2 = || EnumPartialEq::<NotPartialEq>::Struct2 {
        foo: PhantomData,
        bar: 2,
    };

    let struct_2_3 = || EnumPartialEq::<NotPartialEq>::Struct2 {
        foo: PhantomData,
        bar: 3,
    };

    let tuple_0 = EnumPartialEq::<NotPartialEq>::Tuple0;
    let tuple_1 = || EnumPartialEq::<NotPartialEq>::Tuple1(PhantomData);
    let tuple_2_2 = || EnumPartialEq::<NotPartialEq>::Tuple2(PhantomData, 2);
    let tuple_2_3 = || EnumPartialEq::<NotPartialEq>::Tuple2(PhantomData, 3);
    let unit = || EnumPartialEq::<NotPartialEq>::Unit;

    let all_values = [
        struct_0, struct_1, struct_2_2, struct_2_3, tuple_0, tuple_1, tuple_2_2, tuple_2_3, unit,
    ];

    for (i, lhs) in all_values.iter().enumerate() {
        for (j, rhs) in all_values.iter().enumerate() {
            assert_eq!(partial_eq(&lhs(), &rhs()), (i == j, i != j));
        }
    }
}
