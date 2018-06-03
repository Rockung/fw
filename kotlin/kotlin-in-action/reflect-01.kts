import kotlin.reflect.*

class Person(val name: String, val age: Int)

val person = Person("Alice", 29)
val kClass = person.javaClass.kotlin
println(kClass.simpleName)
kClass.memberProperties.forEach { println(it.name) }