use std::cmp::PartialEq;
use std::marker::PhantomData;

struct NotPartialEq;

// Unit.

#[derive(force_derive_impl::PartialEq)]
struct UnitPartialEq1;

#[derive(force_derive_impl::PartialEq)]
struct UnitPartialEq2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq0();

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq1<T>(PhantomData<T>);

#[derive(force_derive_impl::PartialEq)]
struct TuplePartialEq2<T>(PhantomData<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEqEmpty {}

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEqSingle<T> {
    B(PhantomData<T>),
}

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEq1<T> {
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

#[derive(force_derive_impl::PartialEq)]
enum EnumPartialEq2<T>
where
    T: Send + ?Sized,
{
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitPartialEq1: PartialEq);
static_assertions::assert_impl_all!(UnitPartialEq2: PartialEq);
static_assertions::assert_impl_all!(TuplePartialEq1<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(TuplePartialEq2<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEqEmpty: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEqSingle<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEq1<NotPartialEq>: PartialEq);
static_assertions::assert_impl_all!(EnumPartialEq2<NotPartialEq>: PartialEq);

fn partial_eq<T>(lhs: &T, rhs: &T) -> (bool, bool)
where
    T: PartialEq,
{
    (PartialEq::eq(lhs, rhs), PartialEq::ne(lhs, rhs))
}

#[test]
fn test_partial_eq_unit() {
    assert_eq!(partial_eq(&UnitPartialEq1, &UnitPartialEq1), (true, false));
    assert_eq!(partial_eq(&UnitPartialEq2, &UnitPartialEq2), (true, false));
}

#[test]
fn test_partial_eq_tuple() {
    assert_eq!(
        partial_eq(&TuplePartialEq0(), &TuplePartialEq0()),
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
            &TuplePartialEq2::<NotPartialEq>(PhantomData),
            &TuplePartialEq2::<NotPartialEq>(PhantomData),
        ),
        (true, false),
    );
}

#[test]
fn test_partial_eq_enum_single() {
    let b = EnumPartialEqSingle::<NotPartialEq>::B(PhantomData);

    assert_eq!(partial_eq(&b, &b), (true, false));
}

#[test]
fn test_partial_eq_enum_1() {
    let a = EnumPartialEq1::<NotPartialEq>::A;
    let b = EnumPartialEq1::<NotPartialEq>::B(PhantomData);
    let c = EnumPartialEq1::<NotPartialEq>::C { bar: PhantomData };

    // A.

    assert_eq!(partial_eq(&a, &a), (true, false));
    assert_eq!(partial_eq(&a, &b), (false, true));
    assert_eq!(partial_eq(&a, &c), (false, true));

    // B.

    assert_eq!(partial_eq(&b, &a), (false, true));
    assert_eq!(partial_eq(&b, &b), (true, false));
    assert_eq!(partial_eq(&b, &c), (false, true));

    // C.

    assert_eq!(partial_eq(&c, &a), (false, true));
    assert_eq!(partial_eq(&c, &b), (false, true));
    assert_eq!(partial_eq(&c, &c), (true, false));
}

#[test]
fn test_partial_eq_enum_2() {
    let a = EnumPartialEq2::<NotPartialEq>::A;
    let b = EnumPartialEq2::<NotPartialEq>::B(PhantomData);
    let c = EnumPartialEq2::<NotPartialEq>::C { bar: PhantomData };

    // A.

    assert_eq!(partial_eq(&a, &a), (true, false));
    assert_eq!(partial_eq(&a, &b), (false, true));
    assert_eq!(partial_eq(&a, &c), (false, true));

    // B.

    assert_eq!(partial_eq(&b, &a), (false, true));
    assert_eq!(partial_eq(&b, &b), (true, false));
    assert_eq!(partial_eq(&b, &c), (false, true));

    // C.

    assert_eq!(partial_eq(&c, &a), (false, true));
    assert_eq!(partial_eq(&c, &b), (false, true));
    assert_eq!(partial_eq(&c, &c), (true, false));
}
