class Bob {
  def hey(s: String): String =
    if (s.exists(_.isLetter) && s.forall(c => !c.isLetter || c.isUpper))
      "Whoa, chill out!"
    else if (s.endsWith("?"))
      "Sure."
    else if (s.trim.isEmpty)
      "Fine. Be that way!"
    else
      "Whatever."
}
