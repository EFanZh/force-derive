use std::marker::PhantomData;
use std::rc::Rc;

struct NotClone;

// Struct.

#[derive(force_derive_impl::Clone)]
struct StructClone0 {}

#[derive(force_derive_impl::Clone)]
struct StructClone1<T> {
    foo: Rc<T>,
}

#[derive(force_derive_impl::Clone)]
struct StructClone2<T>
where
    u32: Copy,
{
    foo: Rc<T>,
    bar: PhantomData<T>,
}

// Tuple.

#[derive(force_derive_impl::Clone)]
struct TupleClone0();

#[derive(force_derive_impl::Clone)]
struct TupleClone1<T>(Rc<T>);

#[derive(force_derive_impl::Clone)]
struct TupleClone2<T>(Rc<T>, PhantomData<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Clone)]
struct UnitClone;

// Enum.

#[derive(force_derive_impl::Clone)]
enum EnumClone0 {}

#[derive(force_derive_impl::Clone)]
enum EnumClone1<T> {
    Tuple1(Rc<T>),
}

#[derive(force_derive_impl::Clone)]
enum EnumClone<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 { foo: Rc<T> },
    Struct2 { foo: Rc<T>, bar: PhantomData<T> },
    Tuple0(),
    Tuple1(Rc<T>),
    Tuple2(Rc<T>, PhantomData<T>),
    Unit,
}

// Union.

#[allow(dead_code)]
#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
union Union {
    foo: u32,
}

// Tests.

static_assertions::assert_impl_all!(StructClone0: Clone);
static_assertions::assert_impl_all!(StructClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(StructClone2<NotClone>: Clone);
static_assertions::assert_impl_all!(TupleClone0: Clone);
static_assertions::assert_impl_all!(TupleClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(TupleClone2<NotClone>: Clone);
static_assertions::assert_impl_all!(UnitClone: Clone);
static_assertions::assert_impl_all!(EnumClone0: Clone);
static_assertions::assert_impl_all!(EnumClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(EnumClone<NotClone>: Clone);
static_assertions::assert_impl_all!(Union: Clone);

fn clone<T>(value: &T) -> T
where
    T: Clone,
{
    value.clone()
}

#[test]
fn test_clone() {
    // Struct.

    assert!(matches!(clone(&StructClone0 {}), StructClone0 {}));

    assert!(matches!(
        clone(&StructClone1::<NotClone> { foo: Rc::new(NotClone) }),
        StructClone1::<NotClone> { foo } if matches!(*foo, NotClone),
    ));

    assert!(matches!(
        clone(&StructClone2::<NotClone> { foo: Rc::new(NotClone), bar: PhantomData }),
        StructClone2::<NotClone> { foo, bar: PhantomData } if matches!(*foo, NotClone),
    ));

    // Tuple.

    assert!(matches!(clone(&TupleClone0()), TupleClone0()));

    assert!(matches!(
        clone(&TupleClone1::<NotClone>(Rc::new(NotClone))),
        TupleClone1::<NotClone>(x) if matches!(*x, NotClone),
    ));

    assert!(matches!(
        clone(&TupleClone2::<NotClone>(Rc::new(NotClone), PhantomData)),
        TupleClone2::<NotClone>(x, PhantomData) if matches!(*x, NotClone),
    ));

    // Unit.

    assert!(matches!(clone(&UnitClone), UnitClone));

    // Enum.

    assert!(matches!(
        clone(&EnumClone1::<NotClone>::Tuple1(Rc::new(NotClone))),
        EnumClone1::<NotClone>::Tuple1(x) if matches!(*x, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Struct0 {}),
        EnumClone::<NotClone>::Struct0 {},
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Struct1 { foo: Rc::new(NotClone) }),
        EnumClone::<NotClone>::Struct1 { foo } if matches!(*foo, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Struct2 { foo: Rc::new(NotClone), bar: PhantomData }),
        EnumClone::<NotClone>::Struct2 { foo, bar: PhantomData } if matches!(*foo, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Tuple0()),
        EnumClone::<NotClone>::Tuple0(),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Tuple1(Rc::new(NotClone))),
        EnumClone::<NotClone>::Tuple1(foo) if matches!(*foo, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Tuple2(Rc::new(NotClone), PhantomData)),
        EnumClone::<NotClone>::Tuple2(foo, PhantomData) if matches!(*foo, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone::<NotClone>::Unit),
        EnumClone::<NotClone>::Unit,
    ));
}
