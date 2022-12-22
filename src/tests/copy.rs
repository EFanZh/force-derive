#![allow(dead_code)]

use std::marker::PhantomData;

struct NotCopy;

// Unit.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct UnitCopy1;

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct UnitCopy2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct TupleCopy1<T>(PhantomData<T>);

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
struct TupleCopy2<T>(PhantomData<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
enum EnumCopy1<T> {
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

#[derive(force_derive_impl::Clone, force_derive_impl::Copy)]
enum EnumCopy2<T>
where
    T: Send + ?Sized,
{
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitCopy1: Copy);
static_assertions::assert_impl_all!(UnitCopy2: Copy);
static_assertions::assert_impl_all!(TupleCopy1<NotCopy>: Copy);
static_assertions::assert_impl_all!(TupleCopy2<dyn Send>: Copy);
static_assertions::assert_impl_all!(EnumCopy1<NotCopy>: Copy);
static_assertions::assert_impl_all!(EnumCopy2<dyn Send>: Copy);
