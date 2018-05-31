// constructor reference
//    creating an instance of a class
data class Person(val name: String, val age: Int)

val createPerson = ::Person
val p = createPerson("Alice", 29)
println(p)

// bound references
val p2 = Person("Dmitry", 34)
// one-argument function
val personsAgeFunction = Person::age
println(personsAgeFunction(p2))
// zero-argument function
val dmitrysAgeFunction = p2::age
println(dmitrysAgeFunction())
