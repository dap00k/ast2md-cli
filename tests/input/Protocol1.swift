protocol Vehicle {
    var isOn: Bool { get set }
    func start()
    func stop()
}

class Car: Vehicle {
    var isOn: Bool

    init(isOn: Bool){
        self.isOn = isOn
    }
    
    func start() {
        isOn = true
    }
    
    func stop() {
        isOn = false
    }
}

class MyCar {
    let myCar: Vehicle = Car(isOn:false)

    func carOperations() {
        myCar.start()
        myCar.stop()
        print("My car")
    }
}

