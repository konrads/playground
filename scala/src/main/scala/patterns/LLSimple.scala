package patterns

/**
  * LL == LinkedList
  */
//sealed trait LL[T]
//
//case class Node[T](head: T, tail: LL[T]) extends LL[T] {
//  def ::[T](elem: T) =
//    Node(elem, this)
//}
//
//case class Empty() extends LL[Nothing]
//
//object LL {
//  def apply[T](elems: T*): LL[T] = {
//    case Nil => Empty
//    case _ => Node(elems.head, apply(elems.tail: _*))
//  }
//
//  //  def apply[T](elems : T*): LL[T] = {
//  //    if (elems.isEmpty) {
//  //      Empty
//  //    } else {
//  //      Node(elems.head, apply(elems.tail : _*) )
//  //    }
//  //  }
//}
//
//object LLRunner {
//  def main(args: Array[String]) = {
//    println("hello")
//  }
//}
