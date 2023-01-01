#![allow(dead_code)]

use std::marker::PhantomData;

struct NotEq;

// Struct.

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct StructCopy0 {}

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct StructCopy1<T> {
    foo: PhantomData<T>,
}

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct StructCopy2<T>
where
    u32: Copy,
{
    foo: PhantomData<T>,
    bar: PhantomData<T>,
}

// Tuple.

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct TupleCopy0();

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct TupleCopy1<T>(PhantomData<T>);

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct TupleCopy2<T>(PhantomData<T>, PhantomData<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
struct UnitCopy;

// Enum.

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
enum EnumCopy0 {}

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
enum EnumCopy1<T> {
    Tuple1(PhantomData<T>),
}

#[derive(force_derive_impl::Eq, force_derive_impl::PartialEq)]
enum EnumCopy<T>
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

// Tests.

static_assertions::assert_impl_all!(StructCopy0: Eq);
static_assertions::assert_impl_all!(StructCopy1<NotEq>: Eq);
static_assertions::assert_impl_all!(StructCopy2<NotEq>: Eq);
static_assertions::assert_impl_all!(TupleCopy0: Eq);
static_assertions::assert_impl_all!(TupleCopy1<NotEq>: Eq);
static_assertions::assert_impl_all!(TupleCopy2<NotEq>: Eq);
static_assertions::assert_impl_all!(UnitCopy: Eq);
static_assertions::assert_impl_all!(EnumCopy0: Eq);
static_assertions::assert_impl_all!(EnumCopy1<NotEq>: Eq);
static_assertions::assert_impl_all!(EnumCopy<NotEq>: Eq);
