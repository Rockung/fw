import java.math.BigDecimal

// data class Point(val x: Int, val y: Int) {
//     // defines an operator function named `plus`
//     operator fun plus(other: Point): Point {
//         return Point(x + other.x, y + other.y)
//     }
// }

data class Point(val x: Int, val y: Int)
// binary operator function with extension function
operator fun Point.plus(other: Point): Point {
    return Point(x + other.x, y + other.y)
}

val p1 = Point(10, 20)
val p2 = Point(30, 40)
println(p1 + p2)

// compound assignment operators
// operator fun <T> MutableCollection<T>.plusAssign(element: T) {
//     this.add(element)
// }
var p3 = Point(1, 2)
p3 += Point(3, 4)
println(p3)

val list = arrayListOf(1, 2)
list += 3
val newList = list + listOf(4, 5)
println(list)
println(newList)

// unary operator function with extension function
operator fun Point.unaryMinus(): Point {
    return Point(-x, -y)
}
val p = Point(10, 20)
println(-p)

// operator fun BigDecimal.inc() = this + BigDecimal.ONE
var bd = BigDecimal.ZERO
println(bd++) // println(bd), bd += 1
println(++bd) // bd += 1, println(bd)


// comparison operators
// class Point(val x: Int, val y: Int) {
//     override fun Point.equals(obj: Any?): Boolean {
//         if (obj === this) return true
//         if (obj !is Point) return false
//         return obj.x == x && obj.y == y
//     }
// }
println(Point(10, 20) == Point(10, 20))
println(Point(10, 20) != Point(5, 5))
println(null == Point(1, 2))

// binary operators
// a * b | times
// a / b | div
// a % b | mod
// a + b | plus
// a - b | minus

// unary operators
// +a       | unaryPlus
// -a       | unaryMinus
// !a       | not
// ++a, a++ | inc
// --a, a-- | dec
