package langfeatures

/**
  * Look at LangFeaturesTest!
  */
object PartFunction {

  def main(args: Array[String]) {
    // test partialfunction receive
    assert(translate("hello") == "hello back")
    assert(translate(123) == "unexpected 123")
  }

  def translate: PartialFunction[Any, String] = {
    case "hello" => "hello back"
    case x => s"unexpected $x"
  }

}