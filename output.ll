; ModuleID = 'main'
source_filename = "main"

@formatStr = private constant [4 x i8] c"%d\0A\00"
@myStruct = global { i32, i32 } { i32 10, i32 10 }

declare i32 @printf(i8*, ...)

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
  %printfTemp = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStr, i32 0, i32 0), i32 1919)
  ret i32 %printfTemp
}
