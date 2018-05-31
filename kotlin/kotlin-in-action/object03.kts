class User private constructor(val nickname: String) {
    companion object {
        fun newSubscribingUser(email: String) =
            User(email.substringBefore('@'))

        fun newFacebookUser(accountId: Int) =
            User(getFacebookName(accountId))

        private fun getFacebookName(accountId: Int): String {
            return "FacebookName: " + accountId
        }
    }
}

val subscribingUser = User.newSubscribingUser("bob@gmail.com")
println(subscribingUser.nickname)

