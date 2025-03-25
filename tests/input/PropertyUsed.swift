struct Type2 {
  var b: Int
}
struct Type1 {
  var a: Type2
  var c: Int
  mutating func doSomethingElse() {
      c = a.b
  }
}