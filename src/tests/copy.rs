#![allow(dead_code)]

use std::marker::PhantomData;

struct NotCopy;

// Struct.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct StructCopy0 {}

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct StructCopy1<T> {
    foo: PhantomData<T>,
}

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct StructCopy2<T>
where
    u32: Copy,
{
    foo: PhantomData<T>,
    bar: PhantomData<T>,
}

// Tuple.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct TupleCopy0();

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct TupleCopy1<T>(PhantomData<T>);

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct TupleCopy2<T>(PhantomData<T>, PhantomData<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct UnitCopy;

// Enum.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
enum EnumCopy0 {}

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
enum EnumCopy1<T> {
    Tuple1(PhantomData<T>),
}

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
enum EnumCopy<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 {
        foo: PhantomData<T>,
    },
    Struct2 {
        foo: PhantomData<T>,
        bar: PhantomData<T>,
    },
    Tuple0(),
    Tuple1(PhantomData<T>),
    Tuple2(PhantomData<T>, PhantomData<T>),
    Unit,
}

// Tests.

static_assertions::assert_impl_all!(StructCopy0: Copy);
static_assertions::assert_impl_all!(StructCopy1<NotCopy>: Copy);
static_assertions::assert_impl_all!(StructCopy2<NotCopy>: Copy);
static_assertions::assert_impl_all!(TupleCopy0: Copy);
static_assertions::assert_impl_all!(TupleCopy1<NotCopy>: Copy);
static_assertions::assert_impl_all!(TupleCopy2<NotCopy>: Copy);
static_assertions::assert_impl_all!(UnitCopy: Copy);
static_assertions::assert_impl_all!(EnumCopy0: Copy);
static_assertions::assert_impl_all!(EnumCopy1<NotCopy>: Copy);
static_assertions::assert_impl_all!(EnumCopy<NotCopy>: Copy);
