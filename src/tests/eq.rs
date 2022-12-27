#![allow(dead_code)]

use std::marker::PhantomData;

struct NotEq;

// Unit.

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
struct UnitEq1;

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
struct UnitEq2
where
    u32: Eq;

// Tuple.

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
struct TupleEq1<T>(PhantomData<T>);

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
struct TupleEq2<T>(PhantomData<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
enum EnumEqEmpty {}

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
enum EnumEqSingle<T> {
    B(PhantomData<T>),
}

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
enum EnumEq1<T> {
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

#[derive(force_derive_impl::PartialEq, force_derive_impl::Eq)]
enum EnumEq2<T>
where
    T: Send + ?Sized,
{
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitEq1: Eq);
static_assertions::assert_impl_all!(UnitEq2: Eq);
static_assertions::assert_impl_all!(TupleEq1<NotEq>: Eq);
static_assertions::assert_impl_all!(TupleEq2<NotEq>: Eq);
static_assertions::assert_impl_all!(EnumEqEmpty: Eq);
static_assertions::assert_impl_all!(EnumEqSingle<NotEq>: Eq);
static_assertions::assert_impl_all!(EnumEq1<NotEq>: Eq);
static_assertions::assert_impl_all!(EnumEq2<NotEq>: Eq);
