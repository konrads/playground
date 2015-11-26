package langfeatures

object Dog extends Enumeration {
  val Cocker, Springer, Lab = Value
}


object Enums {
  def main(args: Array[String]) {
    println(s"Dogs: ${Dog.Cocker}, ${Dog.Springer}, ${Dog.Lab}")
    println(s"Dog ids: ${Dog.Cocker.id}, ${Dog.Springer.id}, ${Dog.Lab.id}")
    assert(Dog.Cocker.id != Dog.Springer.id)
  }
}
