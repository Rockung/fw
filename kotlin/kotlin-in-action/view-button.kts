open class View {
    open fun click() = println("View clicked")
}

class Button: View() {
    override fun click() = println("Button clicked")
}

// determines the method to call based on the actual value of "view"
val view: View = Button()
view.click()

fun View.showOff() = println("I'm a view!")
fun Button.showOff() = println("I'm a button!")

// the extension function is resolved statically
val view2: View = Button()
view2.showOff()
