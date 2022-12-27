use std::rc::Rc;

struct NotClone;

// Unit.

#[derive(force_derive_impl::Clone)]
struct UnitClone1;

#[derive(force_derive_impl::Clone)]
struct UnitClone2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::Clone)]
struct TupleClone1<T>(Rc<T>);

#[derive(force_derive_impl::Clone)]
struct TupleClone2<T>(Rc<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::Clone)]
enum EnumCloneEmpty {}

#[derive(force_derive_impl::Clone)]
enum EnumCloneSingle<T> {
    B(Rc<T>),
}

#[derive(force_derive_impl::Clone)]
enum EnumClone1<T> {
    A,
    B(Rc<T>),
    C { bar: Rc<T> },
}

#[derive(force_derive_impl::Clone)]
enum EnumClone2<T>
where
    T: Send + ?Sized,
{
    A,
    B(Rc<T>),
    C { bar: Rc<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitClone1: Clone);
static_assertions::assert_impl_all!(UnitClone2: Clone);
static_assertions::assert_impl_all!(TupleClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(TupleClone2<NotClone>: Clone);
static_assertions::assert_impl_all!(EnumCloneEmpty: Clone);
static_assertions::assert_impl_all!(EnumCloneSingle<NotClone>: Clone);
static_assertions::assert_impl_all!(EnumClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(EnumClone2<NotClone>: Clone);

fn clone<T>(value: &T) -> T
where
    T: Clone,
{
    value.clone()
}

#[test]
fn test_clone() {
    //  Unit.

    assert!(matches!(clone(&UnitClone1), UnitClone1));
    assert!(matches!(clone(&UnitClone2), UnitClone2));

    // Tuple.

    assert!(matches!(clone(&TupleClone1(Rc::new(2))), TupleClone1(x) if *x == 2));
    assert!(matches!(clone(&TupleClone2(Rc::new(2))), TupleClone2(x) if *x == 2));

    // Enum.

    assert!(matches!(
        clone(&EnumCloneSingle::<NotClone>::B(Rc::new(NotClone))),
        EnumCloneSingle::<NotClone>::B(x) if matches!(*x, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone1::<NotClone>::A),
        EnumClone1::<NotClone>::A,
    ));

    assert!(matches!(
        clone(&EnumClone1::B(Rc::new(NotClone))),
        EnumClone1::B(x) if matches!(*x, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone1::C {
            bar: Rc::new(NotClone)
        }),
        EnumClone1::C { bar } if matches!(*bar, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone2::<NotClone>::A),
        EnumClone2::<NotClone>::A,
    ));

    assert!(matches!(
        clone(&EnumClone2::B(Rc::new(NotClone))),
        EnumClone2::B(x) if matches!(*x, NotClone),
    ));

    assert!(matches!(
        clone(&EnumClone2::C {
            bar: Rc::new(NotClone)
        }),
        EnumClone2::C { bar } if matches!(*bar, NotClone),
    ));
}
