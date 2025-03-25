
class Type1 {
 
}
 
class Type3 {
   
}
class Type2 {
    var prop1: Type3
    var prop2: Type1
 
    init(prop1: Type3, prop2: Type1) {
        self.prop1 = prop1
        self.prop2 = prop2
    }
}