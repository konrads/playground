package langfeatures

import akka.actor.ActorSystem
import akka.testkit.{ImplicitSender, TestKit}
import org.scalatest.{BeforeAndAfterAll, FlatSpecLike, Matchers}

import scala.concurrent.Await
import scala.concurrent.duration.Duration

class CalcServiceSpec(_system: ActorSystem)
  extends TestKit(_system)
  with FlatSpecLike
  with Matchers
  with BeforeAndAfterAll
  with ImplicitSender {

  def this() = this(ActorSystem("CalcServiceSpec"))

  override def afterAll: Unit = Await.ready(system.terminate(), Duration.Inf)

  "CalcActor" should "add" in {
    val e = SubExpr(Const(6), Add , Const(2))
    CalcService.calc(e) shouldEqual 8
  }
  it should "subtract" in {
    val e = SubExpr(Const(6), Min , Const(2))
    CalcService.calc(e) shouldEqual 4
  }
  it should "multiply" in {
    val e = SubExpr(Const(6), Mul , Const(2))
    CalcService.calc(e) shouldEqual 12
  }
  it should "divide" in {
    val e = SubExpr(Const(6), Div , Const(2))
    CalcService.calc(e) shouldEqual 3
  }

}
