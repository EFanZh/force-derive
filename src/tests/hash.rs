use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;

struct NotHash;

#[derive(Default)]
struct ForceHash<T>(PhantomData<T>);

impl<T> Hash for ForceHash<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&[2]);
        state.write_u8(3);
        state.write_u16(5);
        state.write_u32(7);
        state.write_u64(11);
        state.write_u128(13);
        state.write_usize(17);
        state.write_i8(19);
        state.write_i16(23);
        state.write_i32(29);
        state.write_i64(31);
        state.write_i128(37);
        state.write_isize(41);
    }
}

// Struct.

#[derive(force_derive_impl::Hash)]
struct StructHash0 {}

#[derive(force_derive_impl::Hash)]
struct StructHash1<T> {
    foo: ForceHash<T>,
}

#[derive(force_derive_impl::Hash)]
struct StructHash2<T>
where
    u32: Copy,
{
    foo: ForceHash<T>,
    bar: ForceHash<T>,
}

// Tuple.

#[derive(force_derive_impl::Hash)]
struct TupleHash0();

#[derive(force_derive_impl::Hash)]
struct TupleHash1<T>(ForceHash<T>);

#[derive(force_derive_impl::Hash)]
struct TupleHash2<T>(ForceHash<T>, ForceHash<T>)
where
    u32: Copy;

// Unit.

#[derive(force_derive_impl::Hash)]
struct UnitHash;

// Enum.

#[derive(force_derive_impl::Hash)]
enum EnumHash0 {}

#[derive(force_derive_impl::Hash)]
enum EnumHash1<T> {
    Tuple1(ForceHash<T>),
}

#[derive(force_derive_impl::Hash)]
enum EnumHash<T>
where
    u32: Copy,
{
    Struct0 {},
    Struct1 { foo: ForceHash<T> },
    Struct2 { foo: ForceHash<T>, bar: ForceHash<T> },
    Tuple0(),
    Tuple1(ForceHash<T>),
    Tuple2(ForceHash<T>, ForceHash<T>),
    Unit,
}

// Special identifiers.

#[derive(force_derive_impl::Hash)]
struct SpecialIdentifierStructHash {
    state: u32,
}

#[derive(force_derive_impl::Hash)]
enum SpecialIdentifierEnumHash {
    Struct { state: u32 },
}

// Tests.

static_assertions::assert_impl_all!(StructHash0: Hash);
static_assertions::assert_impl_all!(StructHash1<NotHash>: Hash);
static_assertions::assert_impl_all!(StructHash2<NotHash>: Hash);
static_assertions::assert_impl_all!(TupleHash0: Hash);
static_assertions::assert_impl_all!(TupleHash1<NotHash>: Hash);
static_assertions::assert_impl_all!(TupleHash2<NotHash>: Hash);
static_assertions::assert_impl_all!(UnitHash: Hash);
static_assertions::assert_impl_all!(EnumHash0: Hash);
static_assertions::assert_impl_all!(EnumHash1<NotHash>: Hash);
static_assertions::assert_impl_all!(EnumHash<NotHash>: Hash);

#[derive(PartialEq, Debug, Eq)]
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
    let atom = &ForceHash(PhantomData::<NotHash>);

    // Struct.

    assert_eq!(hash(&StructHash0 {}), []);

    assert_eq!(
        hash(&StructHash1::<NotHash> {
            foo: ForceHash(PhantomData)
        }),
        hash(atom),
    );

    assert_eq!(
        hash(&StructHash2::<NotHash> {
            foo: ForceHash(PhantomData),
            bar: ForceHash(PhantomData),
        }),
        hash_items(&[atom, atom]),
    );

    // Tuple.

    assert_eq!(hash(&TupleHash0()), []);

    assert_eq!(hash(&TupleHash1::<NotHash>(ForceHash(PhantomData))), hash(atom));

    assert_eq!(
        hash(&TupleHash2::<NotHash>(ForceHash(PhantomData), ForceHash(PhantomData))),
        hash_items(&[atom, atom]),
    );

    // Unit.

    assert_eq!(hash(&UnitHash), []);

    // Enum.

    assert_eq!(hash(&EnumHash1::<NotHash>::Tuple1(ForceHash(PhantomData))), hash(atom));

    assert_eq!(
        hash(&EnumHash::<NotHash>::Struct0 {}),
        hash(&mem::discriminant(&EnumHash::<NotHash>::Struct0 {})),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Struct1 {
            foo: ForceHash(PhantomData)
        }),
        hash_items(&[
            &mem::discriminant(&EnumHash::<NotHash>::Struct1 {
                foo: ForceHash(PhantomData)
            }),
            atom,
        ]),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Struct2 {
            foo: ForceHash(PhantomData),
            bar: ForceHash(PhantomData),
        }),
        hash_items(&[
            &mem::discriminant(&EnumHash::<NotHash>::Struct2 {
                foo: ForceHash(PhantomData),
                bar: ForceHash(PhantomData),
            }),
            atom,
            atom,
        ]),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Tuple0()),
        hash(&mem::discriminant(&EnumHash::<NotHash>::Tuple0())),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Tuple1(ForceHash(PhantomData))),
        hash_items(&[
            &mem::discriminant(&EnumHash::<NotHash>::Tuple1(ForceHash(PhantomData))),
            atom,
        ]),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Tuple2(
            ForceHash(PhantomData),
            ForceHash(PhantomData),
        )),
        hash_items(&[
            &mem::discriminant(&EnumHash::<NotHash>::Tuple2(
                ForceHash(PhantomData),
                ForceHash(PhantomData),
            )),
            atom,
            atom,
        ]),
    );

    assert_eq!(
        hash(&EnumHash::<NotHash>::Unit),
        hash(&mem::discriminant(&EnumHash::<NotHash>::Unit)),
    );

    // Special identifiers.

    assert_eq!(hash(&SpecialIdentifierStructHash { state: 2 }), hash(&2_u32));
    assert_eq!(hash(&SpecialIdentifierEnumHash::Struct { state: 2 }), hash(&2_u32));
}
