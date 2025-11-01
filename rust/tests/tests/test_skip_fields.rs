use fory_core::{Fory, ForyDefault, Reader, Serializer, StructSerializer};
use fory_derive::ForyObject;

#[derive(ForyObject, Debug, PartialEq)]
struct TestSkipFields {
    serialized_field: i32,
    #[fory(skip)]
    skipped_field: String,
    another_serialized: f64,
}

#[derive(ForyObject, Debug, PartialEq)]
struct NestedStruct {
    value: i32,
}

#[derive(ForyObject, Debug, PartialEq)]
struct TestNestedSkip {
    normal_field: i32,
    nested: NestedStruct,
    #[fory(skip)]
    skipped_nested: NestedStruct,
}

#[derive(ForyObject, Debug, PartialEq)]
struct MultipleSkipFields {
    field1: i32,
    #[fory(skip)]
    skipped1: String,
    field2: f64,
    #[fory(skip)]
    skipped2: bool,
    field3: f32,
}

#[derive(ForyObject, Debug, PartialEq)]
struct AllFieldsSkipped {
    #[fory(skip)]
    skipped1: String,
    #[fory(skip)]
    skipped2: i32,
    #[fory(skip)]
    skipped3: f64,
}

#[derive(ForyObject, Debug, PartialEq)]
struct ComplexNestedSkip {
    normal_field: i32,
    #[fory(skip)]
    skipped_field: String,
    nested: TestSkipFields,
    #[fory(skip)]
    skipped_nested: TestSkipFields,
}

#[derive(ForyObject, Debug, PartialEq)]
enum TestEnumSkip {
    Pending,
    // #[default]
    Active,
    Inactive,
    #[fory(skip)]
    Deleted,
}

#[test]
fn test_basic_skip_functionality() {
    let mut fory = Fory::default();
    fory.register::<TestSkipFields>(1).unwrap();

    let original = TestSkipFields {
        serialized_field: 42,
        skipped_field: "this should be skipped".to_string(),
        another_serialized: 3.14,
    };

    let bytes = fory.serialize(&original).unwrap();
    let decoded: TestSkipFields = fory.deserialize(&bytes).unwrap();
    assert_eq!(original.serialized_field, decoded.serialized_field);
    assert_eq!(original.another_serialized, decoded.another_serialized);
    assert_eq!(decoded.skipped_field, String::default());

    let mut buf: Vec<u8> = vec![];
    fory.serialize_to(&original, &mut buf).unwrap();
    let mut reader = Reader::new(&buf);
    let decoded: TestSkipFields = fory.deserialize_from(&mut reader).unwrap();
    assert_eq!(original.serialized_field, decoded.serialized_field);
    assert_eq!(original.another_serialized, decoded.another_serialized);
    assert_eq!(decoded.skipped_field, String::default());
}

#[test]
fn test_nested_skip_functionality() {
    let mut fory = Fory::default();
    fory.register::<TestNestedSkip>(2).unwrap();
    fory.register::<NestedStruct>(3).unwrap();

    let original = TestNestedSkip {
        normal_field: 100,
        nested: NestedStruct { value: 200 },
        skipped_nested: NestedStruct { value: 300 },
    };

    let bytes = fory.serialize(&original).unwrap();
    let decoded: TestNestedSkip = fory.deserialize(&bytes).unwrap();

    assert_eq!(original.normal_field, decoded.normal_field);
    assert_eq!(original.nested, decoded.nested);
    assert_eq!(decoded.skipped_nested, NestedStruct::default());
}

#[test]
fn test_multiple_skip_fields() {
    let mut fory = Fory::default();
    fory.register::<MultipleSkipFields>(3).unwrap();

    let original = MultipleSkipFields {
        field1: 42,
        skipped1: "skipped string".to_string(),
        field2: 2.71,
        skipped2: true,
        field3: 255.9,
    };

    let bytes = fory.serialize(&original).unwrap();
    let decoded: MultipleSkipFields = fory.deserialize(&bytes).unwrap();

    assert_eq!(original.field1, decoded.field1);
    assert_eq!(original.field2, decoded.field2);
    assert_eq!(original.field3, decoded.field3);
    assert_eq!(decoded.skipped1, String::default());
    assert_eq!(decoded.skipped2, bool::default());
}

#[test]
fn test_all_fields_skipped() {
    let mut fory = Fory::default();
    fory.register::<AllFieldsSkipped>(4).unwrap();

    let original = AllFieldsSkipped {
        skipped1: "test1".to_string(),
        skipped2: 42,
        skipped3: 3.14,
    };

    let bytes = fory.serialize(&original).unwrap();
    let decoded: AllFieldsSkipped = fory.deserialize(&bytes).unwrap();

    assert_eq!(decoded.skipped1, String::default());
    assert_eq!(decoded.skipped2, i32::default());
    assert_eq!(decoded.skipped3, f64::default());
}

#[test]
fn test_complex_nested_skip() {
    let mut fory = Fory::default();
    fory.register::<ComplexNestedSkip>(5).unwrap();
    fory.register::<TestSkipFields>(6).unwrap();

    let original = ComplexNestedSkip {
        normal_field: 1,
        skipped_field: "should be skipped".to_string(),
        nested: TestSkipFields {
            serialized_field: 2,
            skipped_field: "nested skipped".to_string(),
            another_serialized: 1.41,
        },
        skipped_nested: TestSkipFields {
            serialized_field: 3,
            skipped_field: "completely skipped".to_string(),
            another_serialized: 2.71,
        },
    };

    let bytes = fory.serialize(&original).unwrap();
    let decoded: ComplexNestedSkip = fory.deserialize(&bytes).unwrap();

    assert_eq!(original.normal_field, decoded.normal_field);
    assert_eq!(
        original.nested.serialized_field,
        decoded.nested.serialized_field
    );
    assert_eq!(decoded.nested.skipped_field, String::default());
    assert_eq!(decoded.skipped_field, String::default());
    assert_eq!(decoded.skipped_nested, TestSkipFields::default());
}

#[test]
fn test_enum_skip() {
    let mut fory = Fory::default();
    fory.register::<TestEnumSkip>(6).unwrap();

    let original_v1 = TestEnumSkip::Pending;

    let bytes = fory.serialize(&original_v1).unwrap();
    let decoded: TestEnumSkip = fory.deserialize(&bytes).unwrap();
    assert_eq!(original_v1, decoded);

    let original_skip = TestEnumSkip::Deleted;
    let bytes = fory.serialize(&original_skip).unwrap();
    let decoded: TestEnumSkip = fory.deserialize(&bytes).unwrap();
    assert_eq!(decoded, TestEnumSkip::default());
}

#[test]
fn test_skip_serialization_size() {
    let mut fory = Fory::default();
    fory.register::<TestSkipFields>(10).unwrap();

    let with_skip = TestSkipFields {
        serialized_field: 42,
        skipped_field: "this is a long string that should be skipped".to_string(),
        another_serialized: 3.14,
    };
    #[derive(ForyObject, Debug, PartialEq)]
    struct TestNoSkip {
        serialized_field: i32,
        skipped_field: String,
        another_serialized: f64,
    }

    fory.register::<TestNoSkip>(11).unwrap();

    let without_skip = TestNoSkip {
        serialized_field: 42,
        skipped_field: "this is a long string that should be skipped".to_string(),
        another_serialized: 3.14,
    };

    let bytes_with_skip = fory.serialize(&with_skip).unwrap();
    let bytes_without_skip = fory.serialize(&without_skip).unwrap();

    assert!(
        bytes_with_skip.len() < bytes_without_skip.len(),
        "Skipped version should be smaller: {} < {}",
        bytes_with_skip.len(),
        bytes_without_skip.len()
    );
}

#[test]
fn test_skip_with_different_types() {
    #[derive(ForyObject, Debug, PartialEq)]
    struct MultiTypeSkip {
        field1: i32,
        #[fory(skip)]
        skipped_string: String,
        field2: f64,
        #[fory(skip)]
        skipped_bool: bool,
        field3: i8,
        #[fory(skip)]
        skipped_vec: Vec<i32>,
        field4: i64,
    }

    let mut fory = Fory::default();
    fory.register::<MultiTypeSkip>(12).unwrap();

    let original = MultiTypeSkip {
        field1: 1,
        skipped_string: "test".to_string(),
        field2: 2.0,
        skipped_bool: true,
        field3: 3,
        skipped_vec: vec![1, 2, 3],
        field4: 4,
    };
    let bytes = fory.serialize(&original).unwrap();
    let decoded: MultiTypeSkip = fory.deserialize(&bytes).unwrap();

    assert_eq!(original.field1, decoded.field1);
    assert_eq!(original.field2, decoded.field2);
    assert_eq!(original.field3, decoded.field3);
    assert_eq!(original.field4, decoded.field4);

    assert_eq!(decoded.skipped_string, String::default());
    assert_eq!(decoded.skipped_bool, bool::default());
    assert_eq!(decoded.skipped_vec, Vec::<i32>::default());
}
