/**
  * Representation of Dog breeding, where:
  * - ancestor is Dog
  * - Wolf extends Dog
  * - Coyote extends Dog
  * - Coywolf extends Wolf and Coyote
  */

package patterns

/**
  * My type: subclass of Dog, eg. Wolf
  * I'll mate with another subclass of Dog, T2, eg. Coyote
  * We'll produce a litter of T3, ie. Coywolf, which extends both Wolf and Coyote
  */
abstract class Dog (val howl: String) {
  override def toString() = s"${getClass.getSimpleName}: $howl"
}

class Wolf extends Dog("ooou!")

class Coyote extends Dog("ieee!")

class Coywolf extends Dog("ooouieee!")

class Hyena extends Dog("haha!")

class Family[+T, TFather <: T, TMother <: T, TChild <: TFather](val father: TFather, val mother: TMother, val children: TChild*) {
  override def toString() = s"family: papa: $father, mama: $mother, children: $children"
}

object Family {
  def wolfCoyoteFamily_good(papa: Wolf, mama: Coyote) = {
    new Family(papa, mama, new Coywolf, new Wolf, new Coyote)
  }

  def hyenaFamily_bad(papa: Hyena, mama: Hyena) = {
    new Family(papa, mama, new Coywolf, new Wolf, new Coyote)
  }

  def main(args: Array[String]): Unit = {
    println(wolfCoyoteFamily_good(new Wolf, new Coyote))
    println(hyenaFamily_bad(new Hyena, new Hyena))
  }
}
