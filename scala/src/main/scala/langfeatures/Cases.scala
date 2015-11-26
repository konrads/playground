package langfeatures

trait Case

case class WithIntString(i: Int, s: String) extends Case

case class WithStringInt(s: String, i:Int) extends Case

object Cases {
  def main (args: Array[String]) {
    val c = WithIntString(5, "abc")
    val c2 = c.copy(s = "def")
    println(s"c: ${c}")
    println(s"c2: ${c2}")
    c match {
      case WithIntString(ii, "xyz") => println(s"recognized: xyz -> $ii")
      case WithIntString(ii, ss) => {
        val copy = c.copy(s = "XYZ")
        println(s"other: $ss -> $ii, copy: $copy")
      }
    }
  }
}