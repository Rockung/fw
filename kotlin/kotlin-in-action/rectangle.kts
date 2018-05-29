class Rectangle(val height: Int, val width: Int) {
    val isSquare: Boolean
        get () {
            return height == width
        }
}

val rectangle = Rectangle(41, 43)
println(rectangle.isSquare)
