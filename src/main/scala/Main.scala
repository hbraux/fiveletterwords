import scala.io.Source

object Main {

  private val LETTER_FREQUENCIES = "etaonrishdlfcmugypwbvkjxzq"

  def main(args: Array[String]): Unit = {
    val start = System.currentTimeMillis()
    val words = Source.fromFile("words.txt").getLines().filter( s => s.length == s.toSet.size ).toSeq
    val solution = process("", words)
    println(s"Solution: ${solution.get} found in ${System.currentTimeMillis() - start} ms")
  }

  private def process(current: String, words: Seq[String]): Option[String] = {
    if (current.length >= 25) return Some(current)
    val letter = LETTER_FREQUENCIES.find(!current.contains(_)).get
    val (first, second) = words.partition(_.contains(letter))
    first.map(word => process(word + current, second.filter(w => !word.exists(c => w.contains(c))))).find(_.isDefined).flatten
  }
}
