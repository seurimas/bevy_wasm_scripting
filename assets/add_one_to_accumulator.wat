(module
  (func $get_accumulator (import "env" "get_accumulator") (param i64) (result i32))
  (type $add_one_t (func (param i64) (result i32)))
  (func $add_one_f (type $add_one_t) (param $value i64) (result i32)
    (call $get_accumulator (local.get $value))
    i32.const 1
    i32.add)
  (export "add_one" (func $add_one_f))
  (export "main" (func $add_one_f)))
