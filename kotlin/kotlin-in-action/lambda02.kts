// SAM: single abstract method
fun createAllDoneRunnable(): Runnable {
    return Runnable { println("All done!") }
}

createAllDoneRunnable().run()
