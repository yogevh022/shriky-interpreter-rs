use crate::compiler::byte_operations::OpIndex;
use crate::compiler::code_object::CodeObject;
use crate::compiler::compiler::CompileContext;
use crate::compiler::load::{identity, load_constant};
use crate::compiler::vm_static::*;
use crate::compiler::{ByteOp, Compiler};
use crate::parser::ExprNode;
use crate::parser::nodes::{ClassNode, FunctionNode, ListNode, MapNode};
use crate::runtime::values::{FunctionValue, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn make_map(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    map: MapNode,
) {
    let constant_id = map.id;
    if let Ok(map_const) = Value::try_const_from_map(map.clone()) {
        load_constant(compiler, code_object, constant_id, map_const);
    } else {
        make_runtime_map(compiler, code_object, map);
    }
}

fn make_runtime_map(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    object_node: MapNode,
) {
    let obj_size = object_node.properties.len() * 2;
    object_node.properties.into_iter().for_each(|property| {
        compiler.compile_expr(property.key, &CompileContext::Normal);
        compiler.compile_expr(property.value, &CompileContext::Normal);
    });
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::with_op(ByteOp::MakeMap, obj_size),
    );
}

pub(crate) fn make_list(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    list: ListNode,
) {
    let constant_id = list.id;
    if let Ok(list_const) = Value::try_const_from_list(list.clone()) {
        load_constant(compiler, code_object, constant_id, list_const);
    } else {
        make_runtime_list(compiler, code_object, list);
    }
}

fn make_runtime_list(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    list_node: ListNode,
) {
    let list_len = list_node.elements.len();
    list_node.elements.into_iter().for_each(|element| {
        compiler.compile_expr(element, &CompileContext::Normal);
    });
    compiler.push_op(
        &mut *code_object.borrow_mut(),
        OpIndex::with_op(ByteOp::MakeList, list_len),
    );
}

fn get_function(compiler: &mut Compiler, function_node: FunctionNode) -> FunctionValue {
    let func_code_obj = Rc::new(RefCell::new(CodeObject::from_function(&function_node))); // pre caches params
    compiler.compile_into(
        function_node.body,
        &CompileContext::Function,
        func_code_obj.clone(),
    );
    FunctionValue::new(
        function_node.arguments,
        Rc::try_unwrap(func_code_obj).unwrap().into_inner(),
    )
}

pub(crate) fn make_function(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    function_node: FunctionNode,
) {
    let func_id = function_node.id;
    let func_value = get_function(compiler, function_node);
    let mut_code_obj = &mut *code_object.borrow_mut();
    let func_const_index = cache_constant(mut_code_obj, func_id, Value::Function(func_value));
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::LoadConstant, func_const_index),
    );
}

pub(crate) fn make_method(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    function_node: FunctionNode,
) {
    let mut_code_obj = &mut *code_object.borrow_mut();
    let func_value = get_function(compiler, function_node);
    let method_id = func_value.id;
    let method_value = Value::method(func_value, None); // caller known only at runtime
    let method_const_index = cache_constant(mut_code_obj, method_id, method_value);
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::LoadConstant, method_const_index),
    )
}

pub(crate) fn make_class(
    compiler: &mut Compiler,
    code_object: Rc<RefCell<CodeObject>>,
    class_node: ClassNode,
    context: &CompileContext,
) {
    let class_id = class_node.id;
    let class_inheritance_operand = if let Some(boxed_superclass) = class_node.superclass {
        if let ExprNode::Identity(superclass) = *boxed_superclass {
            identity(compiler, code_object.clone(), superclass, context);
            1usize
        } else {
            panic!("Unexpected superclass: {:?}", *boxed_superclass);
        }
    } else {
        0usize
    };
    let class_value = Value::class(
        None,
        compiler.compile(class_node.body, &CompileContext::Class),
    ); // inherits at runtime
    let class_const_index = cache_constant(&mut *code_object.borrow_mut(), class_id, class_value);
    let mut_code_obj = &mut *code_object.borrow_mut();
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::LoadConstant, class_const_index),
    );
    compiler.push_op(
        mut_code_obj,
        OpIndex::with_op(ByteOp::MakeClass, class_inheritance_operand),
    );
}
