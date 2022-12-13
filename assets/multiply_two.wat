(module
  (type $multiply_two_t (func (param i32) (result i32)))
  (func $multiply_two_f (type $multiply_two_t) (param $value i32) (result i32)
    local.get $value
    i32.const 2
    i32.mul)
  (export "multiply_two" (func $multiply_two_f))
  (export "main" (func $multiply_two_f)))
