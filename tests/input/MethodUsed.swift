struct Type2 {
  func doSomething() {}
}
struct Type1 {
  var a: Type2
  func doSomethingElse() {
    a.doSomething()
  }
}