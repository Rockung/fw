// componentN: destructuring declaration
class Point(val x: Int, val y: Int) {
    operator fun component1() = x
    operator fun component2() = y
}

val p = Point(1, 2)
val (x, y) = p
println(x)
println(y)
