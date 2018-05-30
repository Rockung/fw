interface Clickable {
    fun click()
}

class Button: Clickable {
    override fun click() = println("I was clicked")
}

Button().click()