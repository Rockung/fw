// a[b]: index operator

// get
data class Point(val x: Int, val y: Int)
operator fun Point.get(index: Int): Int {
    return when(index) {
        0 -> x
        1 -> y
        else ->
            throw IndexOutOfBoundsException("Invalid coordinate $index")
    }
}
val p1 = Point(10, 20)
println(p1[1])

// set
data class MutablePoint(var x: Int, var y: Int)
operator fun MutablePoint.set(index: Int, value: Int) {
    when(index) {
        0 -> x = value
        1 -> y = value
        else ->
            throw IndexOutOfBoundsException("Invalid coordinat $index")
    }
}

val p2 = MutablePoint(10, 20)
p2[1] = 42
println(p2)
