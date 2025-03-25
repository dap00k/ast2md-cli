struct Type2 {
  var a: Int
  var b: Int
  func doSomething() {}
  func doSomethingToo() {}
  
}
struct Type1 {
  var c: Type2
  var d: Int
  mutating func doSomethingElse() {
      d = c.a
      d = c.b
      c.doSomething()
      c.doSomethingToo()
  }
}