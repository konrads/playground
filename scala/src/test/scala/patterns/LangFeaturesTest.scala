package patterns

import org.scalatest.{Matchers, FlatSpec}

class LangFeaturesTest extends FlatSpec with Matchers {

  /**
    * Partial function, such as akka receive.  Compatible with case statment... Needs to the type PartialFunction[Any, String]
    */
  def partialTranslate: PartialFunction[Any, String] = {
    case "hello" => "hello back"
    case x => s"unexpected $x"
  }

  def unique(x: Int*) = {
    println("x class: " + x.getClass)
    Set(x: _*)
  }

  def uniqueWrong(x: Int*) =
    Set(x)

  "varargs" should "expand and collapse" in {
    unique(1,1,2,2,3) shouldEqual Set(1,2,3)
  }
  it should "fail if implemented incorrectly" in {
    println("is wrapped array?: " + Seq(1,1,2,2,3).toArray)
    uniqueWrong(1,1,2,2,3) should not equal (Set(1,2,3))  // NOTE: varargs not exploded correctly!
  }

  "partialfuns" should "translate hello" in {
    partialTranslate("hello") shouldEqual "hello back"
  }

  it should "translate non-hello" in {
    partialTranslate(123) shouldEqual "unexpected 123"
  }

//  "variance" should "work..." in {
//    throw new Error("unimplemented")
//  }
//
//  "companion object" should "work..." in {
//    throw new Error("unimplemented")
//  }
//
//  "collections" should "work..." in {
//    throw new Error("unimplemented")
//  }
//
//  "testing frameworks" should "work..." in {
//    throw new Error("unimplemented")
//  }
//
//  "akka" should "work..." in {
//    throw new Error("unimplemented")
//  }

}
