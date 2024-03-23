; ModuleID = 'main.tanu'
source_filename = "main.tanu"

@myStruct = global { i32, i32 } { i32 10, i32 10 }

define i32 @add(i32 %0, i32 %1) {
entry:
  %sumTemp = add i32 %0, %1
  ret i32 %sumTemp
}

define i32 @main() {
entry:
  %addTemp = call i32 @add(i32 50, i32 100)
  ret i32 %addTemp
}
