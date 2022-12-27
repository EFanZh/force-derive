use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;

struct NotHash;

// Unit.

#[derive(force_derive_impl::Hash)]
struct UnitHash1;

#[derive(force_derive_impl::Hash)]
struct UnitHash2
where
    u32: Copy;

// Tuple.

#[derive(force_derive_impl::Hash)]
struct TupleHash0();

#[derive(force_derive_impl::Hash)]
struct TupleHash1<T>(PhantomData<T>);

#[derive(force_derive_impl::Hash)]
struct TupleHash2<T>(PhantomData<T>)
where
    T: Send + ?Sized;

// Enum.

#[derive(force_derive_impl::Hash)]
enum EnumHashEmpty {}

#[derive(force_derive_impl::Hash)]
enum EnumHashSingle<T> {
    A(PhantomData<T>),
}

#[derive(force_derive_impl::Hash)]
enum EnumHash1<T> {
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

#[derive(force_derive_impl::Hash)]
enum EnumHash2<T>
where
    T: Send + ?Sized,
{
    A,
    B(PhantomData<T>),
    C { bar: PhantomData<T> },
}

// Tests.

static_assertions::assert_impl_all!(UnitHash1: Hash);
static_assertions::assert_impl_all!(UnitHash2: Hash);
static_assertions::assert_impl_all!(TupleHash0: Hash);
static_assertions::assert_impl_all!(TupleHash1<NotHash>: Hash);
static_assertions::assert_impl_all!(TupleHash2<NotHash>: Hash);
static_assertions::assert_impl_all!(EnumHashEmpty: Hash);
static_assertions::assert_impl_all!(EnumHashSingle<NotHash>: Hash);
static_assertions::assert_impl_all!(EnumHash1<NotHash>: Hash);
static_assertions::assert_impl_all!(EnumHash2<NotHash>: Hash);
static_assertions::assert_impl_all!(EnumHash2<NotHash>: Hash);

#[derive(PartialEq, Eq)]
enum Operation {
    Write(Box<[u8]>),
    WriteU8(u8),
    WriteU16(u16),
    WriteU32(u32),
    WriteU64(u64),
    WriteU128(u128),
    WriteUsize(usize),
    WriteI8(i8),
    WriteI16(i16),
    WriteI32(i32),
    WriteI64(i64),
    WriteI128(i128),
    WriteIsize(isize),
}

#[derive(Default)]
struct TestHasher {
    operations: Vec<Operation>,
}

impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.operations.push(Operation::Write(bytes.into()));
    }

    fn write_u8(&mut self, i: u8) {
        self.operations.push(Operation::WriteU8(i));
    }

    fn write_u16(&mut self, i: u16) {
        self.operations.push(Operation::WriteU16(i));
    }

    fn write_u32(&mut self, i: u32) {
        self.operations.push(Operation::WriteU32(i))
    }

    fn write_u64(&mut self, i: u64) {
        self.operations.push(Operation::WriteU64(i))
    }

    fn write_u128(&mut self, i: u128) {
        self.operations.push(Operation::WriteU128(i))
    }

    fn write_usize(&mut self, i: usize) {
        self.operations.push(Operation::WriteUsize(i))
    }

    fn write_i8(&mut self, i: i8) {
        self.operations.push(Operation::WriteI8(i))
    }

    fn write_i16(&mut self, i: i16) {
        self.operations.push(Operation::WriteI16(i))
    }

    fn write_i32(&mut self, i: i32) {
        self.operations.push(Operation::WriteI32(i))
    }

    fn write_i64(&mut self, i: i64) {
        self.operations.push(Operation::WriteI64(i))
    }

    fn write_i128(&mut self, i: i128) {
        self.operations.push(Operation::WriteI128(i))
    }

    fn write_isize(&mut self, i: isize) {
        self.operations.push(Operation::WriteIsize(i))
    }
}

fn hash(value: &impl Hash) -> Vec<Operation> {
    let mut hasher = TestHasher::default();

    value.hash(&mut hasher);

    hasher.operations
}

trait TestHash {
    fn hash(&self, state: &mut TestHasher);
}

impl<T> TestHash for T
where
    T: Hash,
{
    fn hash(&self, state: &mut TestHasher) {
        Hash::hash(self, state);
    }
}

fn hash_items(values: &[&dyn TestHash]) -> Vec<Operation> {
    let mut hasher = TestHasher::default();

    for value in values {
        value.hash(&mut hasher);
    }

    hasher.operations
}

#[test]
fn test_hash() {
    // Unit.

    assert!(hash(&UnitHash1).is_empty());
    assert!(hash(&UnitHash2).is_empty());

    // Tuple.

    assert!(hash(&TupleHash0()).is_empty());
    assert!(hash(&TupleHash1::<NotHash>(PhantomData)) == hash(&PhantomData::<NotHash>));
    assert!(hash(&TupleHash2::<NotHash>(PhantomData)) == hash(&PhantomData::<NotHash>));

    // Enum.

    assert!(hash(&EnumHashSingle::<NotHash>::A(PhantomData)) == hash(&PhantomData::<NotHash>));

    assert!(hash(&EnumHash1::<NotHash>::A) == hash(&mem::discriminant(&EnumHash1::<NotHash>::A)));

    assert!(
        hash(&EnumHash1::<NotHash>::B(PhantomData))
            == hash_items(&[
                &mem::discriminant(&EnumHash1::<NotHash>::B(PhantomData)),
                &PhantomData::<NotHash>
            ])
    );

    assert!(
        hash(&EnumHash1::<NotHash>::C { bar: PhantomData })
            == hash_items(&[
                &mem::discriminant(&EnumHash1::<NotHash>::C { bar: PhantomData }),
                &PhantomData::<NotHash>
            ])
    );

    assert!(hash(&EnumHash2::<NotHash>::A) == hash(&mem::discriminant(&EnumHash2::<NotHash>::A)));

    assert!(
        hash(&EnumHash2::<NotHash>::B(PhantomData))
            == hash_items(&[
                &mem::discriminant(&EnumHash2::<NotHash>::B(PhantomData)),
                &PhantomData::<NotHash>
            ])
    );

    assert!(
        hash(&EnumHash2::<NotHash>::C { bar: PhantomData })
            == hash_items(&[
                &mem::discriminant(&EnumHash2::<NotHash>::C { bar: PhantomData }),
                &PhantomData::<NotHash>
            ])
    );
}
