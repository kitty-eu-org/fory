use fory_core::{Fory, ForyDefault, Reader, Serializer, StructSerializer};
use fory_derive::ForyObject;

// 测试基本跳过功能
#[derive(ForyObject, Debug, PartialEq)]
struct TestSkipFields {
    serialized_field: i32,
    #[fory(skip)]
    skipped_field: String,
    another_serialized: f64,
}

// 测试嵌套结构体中的跳过
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

#[test]
fn test_basic_skip_functionality() {
    let mut fory = Fory::default();
    fory.register::<TestSkipFields>(1).unwrap();

    let original = TestSkipFields {
        serialized_field: 42,
        skipped_field: "this should be skipped".to_string(),
        another_serialized: 3.14,
    };

    // Serialize
    let bytes = fory.serialize(&original).unwrap();
    // Deserialize
    let decoded: TestSkipFields = fory.deserialize(&bytes).unwrap();
    assert_eq!(original.serialized_field, decoded.serialized_field);
    assert_eq!(original.another_serialized, decoded.another_serialized);
    assert_eq!(decoded.skipped_field, "".to_string());

    // Serialize to specified buffer
    let mut buf: Vec<u8> = vec![];
    fory.serialize_to(&original, &mut buf).unwrap();
    // Deserialize from specified buffer
    let mut reader = Reader::new(&buf);
    let decoded: TestSkipFields = fory.deserialize_from(&mut reader).unwrap();
    assert_eq!(original.serialized_field, decoded.serialized_field);
    assert_eq!(original.another_serialized, decoded.another_serialized);
    assert_eq!(decoded.skipped_field, "".to_string());
}
//
// #[test]
// fn test_skip_with_default_impl() {
//     let default_instance = TestSkipFields::fory_default();
//
//     assert_eq!(default_instance.serialized_field, 0);
//     assert_eq!(default_instance.skipped_field, String::default());
//     assert_eq!(default_instance.another_serialized, 0.0);
// }
//
// #[test]
// fn test_nested_skip() {
//     let original = TestNestedSkip {
//         normal_field: 100,
//         nested: NestedStruct { value: 200 },
//         skipped_nested: NestedStruct { value: 300 },
//     };
//
//     let mut write_context = fory_core::resolver::context::WriteContext::new();
//     original.fory_write_data(&mut write_context).unwrap();
//
//     let mut read_context = fory_core::resolver::context::ReadContext::new(write_context.into_data());
//     let deserialized = TestNestedSkip::fory_read_data(&mut read_context).unwrap();
//
//     assert_eq!(deserialized.normal_field, 100);
//     assert_eq!(deserialized.nested.value, 200);
//     assert_eq!(deserialized.skipped_nested.value, 0); // 应该是默认值
// }
//
// #[test]
// fn test_skip_in_enum() {
//     let original = TestEnum::StructVariant {
//         field: 42,
//         skipped: "skipped".to_string()
//     };
//
//     let mut write_context = fory_core::resolver::context::WriteContext::new();
//     original.fory_write_data(&mut write_context).unwrap();
//
//     let mut read_context = fory_core::resolver::context::ReadContext::new(write_context.into_data());
//     let deserialized = TestEnum::fory_read_data(&mut read_context).unwrap();
//
//     // 对于枚举，我们主要测试不崩溃
//     assert!(matches!(deserialized, TestEnum::StructVariant { .. }));
// }
//
// #[test]
// fn test_field_info_excludes_skipped_fields() {
//     let type_resolver = fory_core::resolver::type_resolver::TypeResolver::new();
//     let field_infos = TestSkipFields::fory_fields_info(&type_resolver).unwrap();
//
//     let field_names: Vec<&str> = field_infos.iter()
//         .map(|info| info.name.as_str())
//         .collect();
//
//     // 跳过的字段不应该出现在字段信息中
//     assert!(field_names.contains(&"serialized_field"));
//     assert!(field_names.contains(&"another_serialized"));
//     assert!(!field_names.contains(&"skipped_field"));
// }
//
// #[test]
// fn test_sorted_field_names_excludes_skipped() {
//     let field_names = TestSkipFields::fory_get_sorted_field_names();
//
//     // 检查字段名顺序和内容
//     assert_eq!(field_names.len(), 2);
//     assert_eq!(field_names[0], "another_serialized");
//     assert_eq!(field_names[1], "serialized_field");
//     // skipped_field 不应该出现
// }
//
// #[test]
// fn test_reserved_space_excludes_skipped() {
//     let space = TestSkipFields::fory_reserved_space();
//     // 这里可以添加更精确的测试，取决于具体实现
//     assert!(space > 0);
// }
