(module
  (type $add_n_t (func (param i32) (result i32)))
  (func $add_n_f (type $add_n_t) (param $value i32) (result i32)
    local.get $value
    i32.const 2
    i32.add)
  (export "add_n" (func $add_n_f))
  (export "main" (func $add_n_f)))
