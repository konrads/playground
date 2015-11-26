package patterns

/**
  * Created by konrad on 20/11/2015.
  */
trait Tree[T] {
  def map(f: (T) => T): Seq[T]
  def reduce(f: (T, T) => T, init: T): T
  // def y(f: => Int) = 10 * f
}

class Branch[T](val branches: Tree[T]*) extends Tree[T] {
  override def map(f: (T) => T): Seq[T] =
    branches.map(_ map f).flatten
    // (for (b <- branches) yield b map f).flatten

  override def reduce(f: (T, T) => T, init: T): T =
    branches.foldLeft(init)((acc,x) => x reduce(f, acc))
}

class Leaf[T](x: T) extends Tree[T] {
  def map(f: (T) => T): Seq[T] =
    Seq(f(x))

  def reduce(f: (T, T) => T, init: T): T =
    f(x, init)

}

