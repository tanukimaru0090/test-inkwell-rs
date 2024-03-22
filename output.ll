; ModuleID = 'my_module'
source_filename = "my_module"

define i32 @add(i32 %0, i32 %1) {
entry:
  %sumTemp = add i32 %0, %1
  ret i32 %sumTemp
}

define void @main() {
entry:
  ret void
}
