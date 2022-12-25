#![allow(dead_code)]

struct NotDefault;

// Unit.

#[derive(force_derive_impl::Default)]
struct UnitDefault1;

#[derive(force_derive_impl::Default)]
struct UnitDefault2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::Default)]
struct TupleDefault1<T>(Vec<T>);

#[derive(force_derive_impl::Default)]
struct TupleDefault2<T>(Vec<T>)
where
    T: Send;

// Enum.

#[derive(force_derive_impl::Default)]
enum EnumDefault1A<T> {
    #[default]
    A,
    B(Vec<T>),
    C {
        bar: Vec<T>,
    },
}

#[derive(force_derive_impl::Default)]
enum EnumDefault1B<T> {
    A,
    #[default]
    B(Vec<T>),
    C {
        bar: Vec<T>,
    },
}

#[derive(force_derive_impl::Default)]
enum EnumDefault1C<T> {
    A,
    B(Vec<T>),
    #[default]
    C {
        bar: Vec<T>,
    },
}

#[derive(force_derive_impl::Default)]
enum EnumDefault2<T>
where
    T: Send,
{
    #[default]
    A,
    B(Vec<T>),
    C {
        bar: Vec<T>,
    },
}

// Tests.

static_assertions::assert_impl_all!(UnitDefault1: Default);
static_assertions::assert_impl_all!(UnitDefault2: Default);
static_assertions::assert_impl_all!(TupleDefault1<NotDefault>: Default);
static_assertions::assert_impl_all!(TupleDefault2<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefault1A<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefault1B<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefault1C<NotDefault>: Default);
static_assertions::assert_impl_all!(EnumDefault2<NotDefault>: Default);

#[test]
fn test_default() {
    assert!(matches!(
        EnumDefault1A::<NotDefault>::default(),
        EnumDefault1A::<NotDefault>::A,
    ));

    assert!(matches!(
        EnumDefault1B::<NotDefault>::default(),
        EnumDefault1B::<NotDefault>::B(..),
    ));

    assert!(matches!(
        EnumDefault1C::<NotDefault>::default(),
        EnumDefault1C::<NotDefault>::C { .. },
    ));

    assert!(matches!(
        EnumDefault2::<NotDefault>::default(),
        EnumDefault2::<NotDefault>::A,
    ));
}
