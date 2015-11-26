package patterns

/**
  * LL == LinkedList
  *
  * NOTE: for variance, write the trait/abstract first, then generate impl methods - much easier than guessing the types...
  *       when in doubt, for trait typye [T], methods might useparams/return type of [T2 >: T]
  */
sealed trait LL[+T] {
  def ::[T2 >: T](elem: T2): LL[T2]
  def toList: List[T]
  def map[T2](f: T => T2): LL[T2]
  def filter(f: T => Boolean): LL[T]
  def foldLeft[T2 >: T](f: (T2,T2) => T2, init: T2): T2
}

case class Node[+T](private val head: T, private val tail: LL[T]) extends LL[T] {
  def ::[T2 >: T](elem: T2) =
    Node(elem, this)
  def toList: List[T] = head :: tail.toList
  def map[T2](f: T => T2) =
    Node(f(head), tail.map(f))
  def filter(f: (T) => Boolean): LL[T] = f(head) match {
    case true => Node(head, tail.filter(f))
    case false => tail.filter(f)
  }
  def foldLeft[T2 >: T](f: (T2, T2) => T2, init: T2): T2 = tail.foldLeft(f, f(init, head))
}

case object Empty extends LL[Nothing] {
  def ::[T](elem: T) =
    Node(elem, this)
  def toList = Nil
  def map[T2](f: Nothing => T2): LL[T2] = this
  def filter(f: (Nothing) => Boolean) = this
  def foldLeft[T2 >: Nothing](f: (T2, T2) => T2, init: T2): T2 = init
}

object LL {
  def apply[T](elems: T*): LL[T] = elems.isEmpty match {
    case true => Empty
    case _ => Node(elems.head, apply(elems.tail: _*))
  }

//  def apply[T](elems : T*): LL[T] = {
//    if (elems.isEmpty) {
//      Empty
//    } else {
//      Node(elems.head, apply(elems.tail : _*))
//    }
//  }
}
