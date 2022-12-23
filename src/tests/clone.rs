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
static_assertions::assert_impl_all!(EnumClone1<NotClone>: Clone);
static_assertions::assert_impl_all!(EnumClone2<NotClone>: Clone);

#[test]
fn test_clone() {
    fn clone<T>(value: &T) -> T
    where
        T: Clone,
    {
        value.clone()
    }

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
