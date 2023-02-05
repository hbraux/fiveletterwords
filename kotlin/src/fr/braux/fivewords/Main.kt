package fr.braux.fivewords

import java.io.File

object Main {

  fun main() {
    val words = File("words.text").readLines().filter { hasSingleLetters(it) }.toMutableSet()
    while (words.isNotEmpty()) {
      val next = pickWord(words)
      complete(next, words)
      words.remove(next)
    }

  }

  private fun complete(phrase: String, words: Set<String>) {
    if (words.isEmpty()) {
      if (phrase.length == 5)
        println("Found: $phrase!")
    } else
      words.filter { hasNoCommonLetters(phrase, it) }.forEach {
        complete(phrase + it, words)
      }
  }

  private fun pickWord(words: Set<String>): String = words.first()

  private fun hasSingleLetters(word: String) = word.toSet().size == word.length

  private fun hasNoCommonLetters(word: String, other: String) = (word + other).toSet().size == (word.length + other.length)
}
