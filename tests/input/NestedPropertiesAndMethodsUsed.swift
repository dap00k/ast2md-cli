struct Type2 {
  struct Type3{
    var a: Int
    var b: Int
    func doSomething() {}
    func doSomethingToo() {}
  }
  
}
struct Type1 {
  var c: Type2.Type3
  var d: Int
  mutating func doSomethingElse() {
      d = c.a
      d = c.b
      d = c.b
      c.doSomething()
      c.doSomething()
      c.doSomethingToo()
  }
}