from wasmtime import Store, Module, Instance, Func, FuncType

store = Store()
module = Module(store.engine, """
  (module
    (func $hello (import "" "hello"))
    (func (export "run") (call $hello))
  )
""")

def say_hello():
    print("Hello from Python!")
hello = Func(store, FuncType([], []), say_hello)

instance = Instance(store, module, [hello])
run = instance.exports["run"]
run()