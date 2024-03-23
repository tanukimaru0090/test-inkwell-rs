; ModuleID = 'main'
source_filename = "main"

@myStruct = global { i32, i32 } { i32 10, i32 10 }

define i32 @add(i32 %0, i32 %1) {
entry:
  %sumTemp = add i32 %0, %1
  ret i32 %sumTemp
}

define i32 @sub(i32 %0, i32 %1) {
entry:
  %sumTemp = sub i32 %0, %1
  ret i32 %sumTemp
}

define i32 @mul(i32 %0, i32 %1) {
entry:
  %sumTemp = mul i32 %0, %1
  ret i32 %sumTemp
}

define i32 @div(i32 %0, i32 %1) {
entry:
  %sumTemp = sdiv i32 %0, %1
  ret i32 %sumTemp
}

define i32 @main() {
entry:
  %addTemp = call i32 @sub(i32 10, i32 5)
  ret i32 %addTemp
}
