package patterns

import java.io.ByteArrayOutputStream

import langfeatures.{LightOffCmd, LightOnCmd, Switch}
import org.scalatest.{Matchers, FlatSpec}

class CmdTest extends FlatSpec with Matchers {
  "Cmd" should "print to stdout" in {
    val baos = new ByteArrayOutputStream
    scala.Console.withOut(baos)({
      val s = new Switch()
      s.execute(new LightOnCmd)
      s.execute(new LightOffCmd)
      s.execute(new LightOnCmd)
    })
    baos.toString shouldEqual "on\noff\non\n"
  }

}
