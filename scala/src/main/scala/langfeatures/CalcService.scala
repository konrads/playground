package langfeatures

import akka.actor._
import akka.pattern.ask
import scala.concurrent.{Await, Future}
import scala.concurrent.duration._
import akka.util.Timeout

trait Op {
  def calc(left: Int, right: Int): Int
}
case object Add extends Op { def calc(left: Int, right: Int) = left + right }
case object Min extends Op { def calc(left: Int, right: Int) = left - right }
case object Mul extends Op { def calc(left: Int, right: Int) = left * right }
case object Div extends Op { def calc(left: Int, right: Int) = left / right }

sealed trait Expr
case class Const(v: Int) extends Expr
case class SubExpr(left: Expr, op: Op, right: Expr) extends Expr

case class Resp(id: Symbol, result: Int)

class CalcActor(id: Symbol, e: Expr, _replyTo: ActorRef = null) extends Actor with ActorLogging {
  private var calculated = Map[Symbol, Option[Int]]()
  private var op: Op = null

  override def preStart(): Unit = e match {
    case c:Const =>
      log.info("created with constant {}, will reply to {}, org _replyTo {}", c.v, replyTo, _replyTo)
      replyTo ! Resp(id, c.v)
    case SubExpr(left, op, right) =>
      log.info("created with subexpr {}, {}, will reply to {}, org _replyTo {}", left, right, replyTo, _replyTo)
      context.actorOf(CalcActor.props('left, left), name = "left")
      context.actorOf(CalcActor.props('right, right), name = "right")
      calculated += 'left -> None
      calculated += 'right -> None
      this.op = op
  }

  override def receive = {
    case Resp(respId, result) =>
      // NOTE: not checking if already received duplicate!
      log.info("response {} - result {}", respId, result)
      calculated += respId -> Some(result)
      val hasNones = calculated.exists(! _._2.isDefined)
      if (! hasNones) {
        val left = calculated('left).get
        val right = calculated('right).get
        val overallResult = op.calc(left, right)
        log.info("replying back to {}: {} with op {}", replyTo, overallResult, op.getClass.getSimpleName)
        replyTo ! Resp(id, overallResult)
        context.stop(self)
      }
  }

  private def replyTo = _replyTo match {
    case null => context.parent
    case _ => _replyTo
  }
}

object CalcActor {
  def props(id: Symbol, expr: Expr, replyTo: ActorRef = null): Props = Props(classOf[CalcActor], id, expr, replyTo)
}

class CalcService extends Actor {
  override def receive = {
    case e: Expr =>
      // replyto sender, not the parent!
      // note, the worker is actually replying, not the CalcService
      context.actorOf(CalcActor.props('calc_actor, e, sender))
  }
}

object CalcService {
  implicit val timeout = Timeout(1 second)
  lazy val system = ActorSystem("calc-system")
  lazy val service = system.actorOf(props, "calc-service")

  def props(): Props = Props(classOf[CalcService])

  def calc(e: Expr): Int = {
    val reply: Future[Resp] = (service ? e).mapTo[Resp]
    val resp = Await.result(reply, timeout.duration)
    resp.result
  }
}
